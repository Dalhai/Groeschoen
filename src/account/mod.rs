//! Accounts tracked by Working Poor.
//!
//! Accounts come in different kinds, modelled as variants of [`Account`]. For
//! now there is exactly one kind; more will be added as the app grows.

/// A financial account.
///
/// Each kind of account is a variant. Additional kinds will be added over time.
#[allow(dead_code)] // Not wired into the UI yet.
#[derive(Debug, Clone, PartialEq)]
pub enum Account {
    /// A basic cash account: a named balance with no associated currency.
    Cash {
        /// Human-readable name of the account.
        name: String,
        /// Current balance. A plain number, no currency attached.
        balance: f64,
    },
}
