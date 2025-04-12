use anyhow::Result;
use log::{error, info};
use dotenv::dotenv;
use std::env;

use crate::processing::kafka::consumer::{ConsumerConfig, TransactionConsumer, TransactionEvent};
use crate::processing::processors::TransactionProcessor;

/// Example of a Kafka consumer for transaction events
#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    env_logger::init();
    
    info!("Starting Transaction Processor Example");
    
    // Get Kafka broker details from env vars
    let kafka_brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka_topics = vec![
        env::var("KAFKA_TOPIC").unwrap_or_else(|_| "eth-transactions".to_string())
    ];
    let kafka_group_id = env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "balance-processor".to_string());
    
    // Configure the consumer
    let consumer_config = ConsumerConfig {
        brokers: kafka_brokers,
        group_id: kafka_group_id,
        topics: kafka_topics,
        auto_offset_reset: "earliest".to_string(),
        ..Default::default()
    };
    
    // Create the consumer
    let consumer = TransactionConsumer::new(consumer_config)?;
    
    // Create the transaction processor
    let processor = TransactionProcessor::new();
    
    info!("Kafka consumer created, starting to process messages");
    
    // Start consuming messages
    consumer.consume::<_, TransactionEvent>(|event| {
        let event_clone = event.clone();
        let processor_clone = processor.clone();
        
        // Spawn a task to process the transaction
        tokio::spawn(async move {
            info!("Processing transaction: {} (block #{})", event_clone.transaction_hash, event_clone.block_number);
            
            if let Err(e) = processor_clone.process_transaction(event_clone).await {
                error!("Failed to process transaction: {}", e);
            }
        });
        
        Ok(())
    }).await?;
    
    Ok(())
}

/// Run with:
/// RUST_LOG=info cargo run --example transaction_processor_example 