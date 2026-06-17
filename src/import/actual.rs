//! Import of the "actual" CSV export format.
//!
//! The export is deserialized into [`ActualRecord`], an intermediate struct that
//! mirrors the file's columns exactly, and then mapped onto the finance data
//! model. Keeping the CSV shape separate from [`Transaction`] means the import
//! format can drift without touching the domain types.

use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::finance::{Category, Payee, Transaction};

/// Path to the sample transactions export used during development.
const SAMPLE_TRANSACTIONS_PATH: &str = "dev/sample_transactions_actual.csv";

/// One row of the actual CSV export, mirroring its columns exactly.
///
/// Only a subset is mapped onto [`Transaction`]; the rest is captured so the
/// full schema is documented and parsing stays strict (unexpected columns would
/// fail to deserialize).
#[derive(Debug, Deserialize)]
#[allow(dead_code)] // Several columns are parsed for completeness but not yet used.
struct ActualRecord {
    #[serde(rename = "Account")]
    account: String,
    #[serde(rename = "Date")]
    date: NaiveDate,
    #[serde(rename = "Payee")]
    payee: String,
    #[serde(rename = "Notes")]
    notes: String,
    #[serde(rename = "Category_Group")]
    category_group: String,
    #[serde(rename = "Category")]
    category: String,
    #[serde(rename = "Amount")]
    amount: f64,
    #[serde(rename = "Split_Amount")]
    split_amount: f64,
    #[serde(rename = "Cleared")]
    cleared: String,
}

impl From<ActualRecord> for Transaction {
    fn from(record: ActualRecord) -> Self {
        Transaction {
            amount: record.amount,
            payee: Payee(record.payee),
            notes: record.notes,
            category: Category(record.category),
            date: record.date,
        }
    }
}

/// Loads sample transactions from the development CSV export.
pub fn load_sample_transactions() -> Result<Vec<Transaction>> {
    let path = SAMPLE_TRANSACTIONS_PATH;

    // The export contains some invalid UTF-8, so decode the whole file lossily
    // up front and feed the cleaned text to the CSV reader.
    let bytes =
        std::fs::read(path).with_context(|| format!("failed to read transactions file `{path}`"))?;
    let text = String::from_utf8_lossy(&bytes);

    let mut reader = csv::Reader::from_reader(text.as_bytes());

    let mut transactions = Vec::new();
    for (row, record) in reader.deserialize::<ActualRecord>().enumerate() {
        let record = record.with_context(|| format!("failed to parse row {row} of `{path}`"))?;
        transactions.push(record.into());
    }

    Ok(transactions)
}
