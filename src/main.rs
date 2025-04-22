mod common;
mod data_collection;
mod processing;
mod query;
mod storage;

use crate::processing::service::ProcessingService;
use anyhow::Result;
use data_collection::hypersync::load_hypersync_config;
use data_collection::kafka::{self, load_kafka_config};
use data_collection::service::DataCollectionService;
use dotenv::dotenv;
use hypersync_client::format::{Address, Hex};
use log::info;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // initialize logging
    env_logger::init();

    info!("Starting Ethereum Balance API - Phase 1");

    // Load configurations
    let hypersync_config = load_hypersync_config();
    let kafka_config = load_kafka_config();
    info!("Loaded configurations");

    // Create data collection service
    let data_collection_service = DataCollectionService::new(hypersync_config, kafka_config.clone())?;


    let weth_address = Address::decode_hex("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2")?;

    // Query a small block range for testing
    let from_block = 4719568u64;
    let to_block = Some(9439136u64);

    info!(
        "Querying transactions for WETH contract from block {} to {:?}",
        from_block, to_block
    );

    // Collect transactions and send to Kafka
    let tx_processed = data_collection_service
        .collect_historical_transactions(weth_address, from_block, to_block)
        .await?;

    info!("Processed {} transactions", tx_processed);

    info!("Data collection test completed successfully");

    // Create processing service
    let processing_service = ProcessingService::new(kafka_config)?;
    processing_service.start().await?;

    // Keep the main thread alive until ctrl+c is received
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutting down gracefully");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    Ok(())
}
