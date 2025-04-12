// Common type definitions

use serde::{Deserialize, Serialize};

/// Represents a blockchain transaction event
// Kafka message schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEvent {
    /// Chain ID of the blockchain network
    pub chain_id: u64,
    /// Transaction hash in hexadecimal format
    pub transaction_hash: String,
    /// Block number where the transaction was included
    pub block_number: u64,
    /// Sender address in hexadecimal format
    pub from_address: String,
    /// Recipient address in hexadecimal format
    pub to_address: String,
    /// Transaction value in wei (as string to handle large numbers)
    pub value: String, // Using String for BigDecimal compatibility
    /// Transaction timestamp (Unix epoch time)
    pub timestamp: u64,
    /// Whether the transaction was successful
    pub is_success: bool,
}
