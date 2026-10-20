use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident, $prefix:literal) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub u64);

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{}", $prefix, self.0)
            }
        }
    };
}

define_id!(
    /// Identifies an account in the chart of accounts.
    AccountId, "A"
);
define_id!(
    /// Identifies a customer.
    CustomerId, "C"
);
define_id!(
    /// Identifies a vendor.
    VendorId, "V"
);
define_id!(
    /// Identifies a product or service item.
    ItemId, "I"
);
define_id!(
    /// Identifies a transaction.
    TxnId, "T"
);
