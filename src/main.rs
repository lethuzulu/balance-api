mod common;
mod data_collection;
mod processing;
mod query;
mod storage;

use crate::processing::service::ProcessingService;
use anyhow::Result;
use data_collection::collection_manager::CollectionManager;
use data_collection::service::DataCollectionService;
use data_collection::{hypersync::load_hypersync_config, kafka::load_kafka_config};
use dotenv::dotenv;
use std::sync::Arc;
// use hypersync_client::format::{Address, Hex};
use log::{error, info};
// use query::graphql;
use tokio::signal;

#[tokio::main(flavor = "multi_thread")]
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

    // Create and start processing service
    let processing_service = ProcessingService::new(kafka_config.clone())?;
    let processing_task = tokio::spawn(async move {
        processing_service.start().await.unwrap();
    });

    // Create data collection service
    let data_collection_service = DataCollectionService::new(hypersync_config, kafka_config)?;
    let redis_client = redis::Client::open("redis://127.0.0.1/")?;
    let collection_manager = Arc::new(CollectionManager::new(
        Arc::new(data_collection_service),
        redis_client,
    ));

    
    // Start GraphQL server
    let api_task = tokio::spawn(async move {
        if let Err(e) = query::graphql::server::start_graphql_server(collection_manager).await {
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
