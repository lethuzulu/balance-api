// Balance record model

use chrono::{DateTime, Utc};
use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct HistoricalBalance {
    pub address: String,
    pub block_number: u64,
    pub balance: String,
    pub timestamp: DateTime<Utc>,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceChange {
    pub address: String,
    pub block_number: u64,
    pub change_amount: String,
    pub transaction_hash: String,
    pub timestamp: DateTime<Utc>,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Row)]
pub struct CurrentBalance {
    pub address: String,
    pub chain_id: u64,
    pub balance: String,
    pub last_block_number: u64,
    pub last_update: DateTime<Utc>,
}
