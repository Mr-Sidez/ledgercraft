use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{parse_scaled, CoreError, CoreResult, ScaleError};

/// An exact quantity with three decimal places, stored as thousandths.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Qty(i64);

impl Default for Qty {
    fn default() -> Self {
        Self::ONE
    }
}

impl Qty {
    pub const ONE: Self = Self(1000);
    pub const ZERO: Self = Self(0);

    pub const fn from_milli(milli: i64) -> Self {
        Self(milli)
    }

    pub fn from_whole(n: i64) -> Self {
        Self(n.saturating_mul(1000))
    }

    pub const fn milli(self) -> i64 {
        self.0
    }

    pub fn parse(input: &str) -> CoreResult<Self> {
        parse_scaled(input, 3).map(Self).map_err(|e| match e {
            ScaleError::Invalid => CoreError::InvalidQty(input.to_string()),
            ScaleError::Overflow => CoreError::Overflow,
        })
    }
}

/// `2`, `2.5`, `0.125`: trailing zeros trimmed.
impl fmt::Display for Qty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let abs = self.0.unsigned_abs();
        let whole = abs / 1000;
        let frac = abs % 1000;
        if frac == 0 {
            write!(f, "{sign}{whole}")
        } else {
            let frac = format!("{frac:03}");
            write!(f, "{sign}{whole}.{}", frac.trim_end_matches('0'))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        assert_eq!(Qty::parse("2.5").unwrap().milli(), 2500);
        assert_eq!(Qty::parse("2.5").unwrap().to_string(), "2.5");
        assert_eq!(Qty::parse("3").unwrap().to_string(), "3");
        assert_eq!(Qty::parse("-0.125").unwrap().to_string(), "-0.125");
        assert!(Qty::parse("1.0005").is_err());
    }
}
