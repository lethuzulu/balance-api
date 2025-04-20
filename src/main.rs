mod common;
mod data_collection;
mod processing;
mod query;
mod storage;

use crate::processing::service::ProcessingService;
use anyhow::Result;
use data_collection::hypersync::load_hypersync_config;
use data_collection::kafka::load_kafka_config;
use data_collection::service::DataCollectionService;
use dotenv::dotenv;
use log::info;

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
    let data_collection_service = DataCollectionService::new(hypersync_config, kafka_config)?;

    // Create processing service
    let processing_service = ProcessingService::new()?;
    processing_service.start().await?;

    Ok(())
}
