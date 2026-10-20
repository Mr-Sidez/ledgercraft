//! L3 of LedgerCraft: the engine.
//!
//! Everything a user can do to their books is a [`Command`]. The desktop UI,
//! the CLI and (later) the agent control channel all go through
//! [`Session::execute`], which validates, applies, records undo history and
//! never leaves the books half-changed.

mod command;
mod sample;
mod session;

pub use command::{Command, CommandInfo, CATALOG};
pub use sample::sample_company;
pub use session::{EngineError, Outcome, Session};

/// A new company with the default chart of accounts.
pub fn new_company(name: &str) -> ledgercraft_ledger::Company {
    ledgercraft_ledger::Company::with_default_accounts(name)
}
