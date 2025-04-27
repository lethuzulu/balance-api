use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceChange {
    pub address: String,
    pub block_number: u64,
    pub amount: String,
    pub transaction_hash: String,
    pub timestamp: DateTime<Utc>,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBalance {
    pub address: String,
    pub chain_id: u64,
    pub balance: String,
    pub block_number: u64,
    pub timestamp: DateTime<Utc>,
}
