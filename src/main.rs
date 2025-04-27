mod common;
mod data_collection;
mod processing;
mod query;
mod storage;

use crate::processing::service::ProcessingService;
use anyhow::Result;
use data_collection::service::DataCollectionService;
use data_collection::{hypersync::load_hypersync_config, kafka::load_kafka_config};
use dotenv::dotenv;
// use hypersync_client::format::{Address, Hex};
use log::{error, info};
// use query::graphql;
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
    let data_collection_service =
        DataCollectionService::new(hypersync_config, kafka_config.clone())?;

    // Start data collection (example for WETH)
    tokio::spawn(async move {
        // let weth_address = Address::decode_hex("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
        // data_collection_service.start_stream(weth_address).await.unwrap();
    });

    // Create and start processing service
    let processing_service = ProcessingService::new(kafka_config)?;
    let processing_task = tokio::spawn(async move {
        processing_service.start().await.unwrap();
    });

    // Start GraphQL server
    let api_task = tokio::spawn(async move {
        if let Err(e) = query::graphql::server::start_graphql_server().await {
            error!("Failed to start GraphQL server: {}", e);
        }
    });

    // Keep the main thread alive until ctrl+c is received
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutting down gracefully");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    // Wait for tasks to complete
    let _ = tokio::join!(processing_task, api_task);

    Ok(())
}
