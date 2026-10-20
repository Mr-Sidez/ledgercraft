use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use ledgercraft_file::FileError;
use ledgercraft_ledger::{Company, LedgerError};

use crate::Command;

const MAX_UNDO: usize = 200;

#[derive(Debug)]
pub enum EngineError {
    Ledger(LedgerError),
    File(FileError),
    BadCommand(String),
    NoPath,
    Internal(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ledger(e) => fmt::Display::fmt(e, f),
            Self::File(e) => fmt::Display::fmt(e, f),
            Self::BadCommand(e) => write!(f, "not a valid command: {e}"),
            Self::NoPath => f.write_str("choose where to save the company file first"),
            Self::Internal(e) => write!(f, "internal error, nothing was changed: {e}"),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<LedgerError> for EngineError {
    fn from(e: LedgerError) -> Self {
        Self::Ledger(e)
    }
}

impl From<FileError> for EngineError {
    fn from(e: FileError) -> Self {
        Self::File(e)
    }
}

/// What a successful command did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    pub command: &'static str,
    /// The id of anything created, e.g. `T42`.
    pub created: Option<String>,
}

/// An open company file with its undo history.
pub struct Session {
    company: Company,
    path: Option<PathBuf>,
    undo: Vec<(String, Company)>,
    redo: Vec<(String, Company)>,
    dirty: bool,
}

impl Session {
    pub fn new(company: Company) -> Self {
        Self { company, path: None, undo: Vec::new(), redo: Vec::new(), dirty: false }
    }

    pub fn open(path: &Path) -> Result<Self, EngineError> {
        let company = ledgercraft_file::load(path)?;
        let mut s = Self::new(company);
        s.path = Some(path.to_path_buf());
        Ok(s)
    }

    pub fn company(&self) -> &Company {
        &self.company
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Window title: `Company — file.lcbooks*`.
    pub fn title(&self) -> String {
        let file = self
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Unsaved".into());
        let star = if self.dirty { "*" } else { "" };
        format!("{} — {file}{star}", self.company.info.name)
    }

    /// Runs a command. On any error — including a bug that panics — the books
    /// are restored exactly as they were.
    pub fn execute(&mut self, cmd: Command) -> Result<Outcome, EngineError> {
        let id = cmd.id();
        let label = cmd.undo_label();
        let before = self.company.clone();
        let result = catch_unwind(AssertUnwindSafe(|| cmd.apply(&mut self.company)));
        match result {
            Ok(Ok(created)) => {
                self.undo.push((label, before));
                if self.undo.len() > MAX_UNDO {
                    self.undo.remove(0);
                }
                self.redo.clear();
                self.dirty = true;
                Ok(Outcome { command: id, created })
            }
            Ok(Err(e)) => {
                self.company = before;
                Err(e.into())
            }
            Err(_) => {
                self.company = before;
                Err(EngineError::Internal(format!("{id} panicked")))
            }
        }
    }

    /// Parses and runs a JSON command.
    pub fn execute_json(&mut self, json: &str) -> Result<Outcome, EngineError> {
        let cmd: Command =
            serde_json::from_str(json).map_err(|e| EngineError::BadCommand(e.to_string()))?;
        self.execute(cmd)
    }

    pub fn undo_label(&self) -> Option<&str> {
        self.undo.last().map(|(l, _)| l.as_str())
    }

    pub fn redo_label(&self) -> Option<&str> {
        self.redo.last().map(|(l, _)| l.as_str())
    }

    /// Undoes the last command, returning its label.
    pub fn undo(&mut self) -> Option<String> {
        let (label, prev) = self.undo.pop()?;
        let current = std::mem::replace(&mut self.company, prev);
        self.redo.push((label.clone(), current));
        self.dirty = true;
        Some(label)
    }

    pub fn redo(&mut self) -> Option<String> {
        let (label, next) = self.redo.pop()?;
        let current = std::mem::replace(&mut self.company, next);
        self.undo.push((label.clone(), current));
        self.dirty = true;
        Some(label)
    }

    pub fn save(&mut self) -> Result<(), EngineError> {
        let path = self.path.clone().ok_or(EngineError::NoPath)?;
        ledgercraft_file::save(&path, &self.company)?;
        self.dirty = false;
        Ok(())
    }

    pub fn save_as(&mut self, path: &Path) -> Result<(), EngineError> {
        let path = ledgercraft_file::with_extension(path);
        ledgercraft_file::save(&path, &self.company)?;
        self.path = Some(path);
        self.dirty = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_commands_change_nothing_and_undo_works() {
        let mut s = Session::new(Company::with_default_accounts("Undo Co"));
        let before = s.company().clone();
        let bad = s.execute_json(r#"{"command":"account.create","name":"Checking","kind":"bank"}"#);
        assert!(bad.is_err());
        assert_eq!(s.company(), &before);
        assert!(!s.is_dirty());

        let ok = s
            .execute_json(r#"{"command":"account.create","number":"6700","name":"Postage","kind":"expense"}"#)
            .unwrap();
        assert!(ok.created.is_some());
        assert!(s.company().account_by_name("Postage").is_some());
        assert_eq!(s.undo().as_deref(), Some("New Account Postage"));
        assert!(s.company().account_by_name("Postage").is_none());
        s.redo();
        assert!(s.company().account_by_name("Postage").is_some());
    }

    #[test]
    fn every_command_rejects_empty_params_without_panicking() {
        for info in crate::CATALOG {
            let mut s = Session::new(Company::with_default_accounts("Fuzz"));
            let json = format!(r#"{{"command":"{}"}}"#, info.id);
            assert!(s.execute_json(&json).is_err(), "{} accepted empty params", info.id);
        }
    }

    #[test]
    fn sample_company_is_valid() {
        let c = crate::sample_company().unwrap();
        assert!(c.transactions.len() > 10);
    }
}
