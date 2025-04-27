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
