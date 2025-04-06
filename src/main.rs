mod data_collection;
mod processing;
mod query;
mod storage;

use anyhow::Result;
use data_collection::hypersync::client::HypersyncClient;
use data_collection::hypersync::config::load_config;
use dotenv::dotenv;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv().ok();

    // initialize logging
    env_logger::init();

    info!("Starting Ethereum Balance API - Phase 1");

    // Load Hypersync configuration
    let hypersync_config = load_config()?;
    info!("Loaded Hypersync configuration");

    // Initialize Hypersynct client
    let client = HypersyncClient::new(hypersync_config)?;
    info!("Initialized Hypersync client");

    Ok(())
}
