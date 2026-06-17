#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    /// Human-readable name of the account.
    pub name: String,

    /// Current balance. A plain number, no currency attached.
    pub balance: f64,

    /// List of transactions associated with this account.
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Payee(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Category(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    /// Change amount, positive for deposits, negative for withdrawals
    pub amount: f64,

    /// Source of the transaction, usually an account
    pub payee: Payee,

    /// Transaction notes, arbitrary additional information about the transaction
    pub notes: String,

    /// Transaction category, used for budgeting and reporting
    pub category: Category,

    /// Date of the transaction
    pub date: chrono::NaiveDate,
}

impl Transaction {
    pub fn is_deposit(&self) -> bool {
        self.amount > 0.0
    }

    pub fn is_withdrawal(&self) -> bool {
        self.amount < 0.0
    }
}
