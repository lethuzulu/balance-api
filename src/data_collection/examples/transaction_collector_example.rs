use anyhow::Result;
use std::env;

use crate::data_collection::hypersync::{
    ClientConfig, HypersyncClient, TransactionCollector,
};

/// Example showing how to use the TransactionCollector
pub async fn run_transaction_collector_example() -> Result<()> {
    // Load configuration from environment variables
    dotenv::dotenv().ok();
    
    // Get Kafka broker details from env vars
    let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka_topic = env::var("KAFKA_TOPIC").unwrap_or_else(|_| "eth-transactions".to_string());
    
    // Initialize Hypersynsc client
    let hypersync_config = ClientConfig::default();
    let hypersync_client = HypersyncClient::new(hypersync_config)?;
    
    // Create transaction collector
    let collector = TransactionCollector::new(
        hypersync_client,
        &kafka_brokers,
        &kafka_topic,
        1, // Chain ID for Ethereum mainnet
    )?;
    
    // Example Ethereum address - USDC contract
    let example_address = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
    
    // Collect historical transactions (limited block range for example)
    let latest_block = collector
        .collect_historical_transactions(example_address, 16_000_000, Some(16_001_000))
        .await?;
    
    println!("Successfully processed transactions up to block: {}", latest_block);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[tokio::test]
    #[ignore] // Ignore by default as it requires external Kafka
    async fn test_transaction_collector_example() {
        let result = run_transaction_collector_example().await;
        assert!(result.is_ok());
    }
} 