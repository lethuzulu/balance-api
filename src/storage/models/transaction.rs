use async_graphql::Data;
// Transaction record model
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::types::TransactionEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub block_number: u64,
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
    pub timestamp: DateTime<Utc>,
    pub chain_id: u64,
    pub is_success: bool,
}

impl From<TransactionEvent> for TransactionRecord {
    fn from(event: TransactionEvent) -> Self {
        Self {
            block_number: event.block_number,
            transaction_hash: event.transaction_hash,
            from_address: event.from_address,
            to_address: event.to_address,
            value: event.value,
            timestamp: DateTime::from_timestamp(event.timestamp as i64, 0).unwrap_or_default(),
            chain_id: event.chain_id,
            is_success: event.is_success,
        }
    }
}
