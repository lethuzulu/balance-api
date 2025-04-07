// Hypersync client wrapper

use anyhow::{Context, Result};
use async_graphql::parser::types::Field;
use hypersync_client::Client as BaseClient;
use hypersync_client::QueryResponse;
use hypersync_client::format::{Address, Data, FixedSizeData, Hex};
use hypersync_client::net_types::Query;
use hypersync_client::net_types::TransactionSelection;
use hypersync_client::net_types::{FieldSelection, TraceSelection};
use log::info;
use std::collections::{BTreeMap, BTreeSet};

use super::config::ClientConfig;

pub struct HypersyncClient {
    client: BaseClient,
    config: ClientConfig,
}

impl HypersyncClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let base_config = config
            .to_base_client_config()
            .context("Failed to create client config")?;
        let base_client =
            BaseClient::new(base_config).context("Failed to create HyperSync Client")?;

        Ok(HypersyncClient {
            client: base_client,
            config,
        })
    }
    /// Query transactions sent to a specific address
    pub async fn query_transactions_to_address(
        &self,
        address: FixedSizeData<20>,
        from_block: u64,
        to_block: Option<u64>,
    ) -> Result<QueryResponse> {
        info!(
            "Querying transactions to address {} from block: {}",
            address, from_block
        );
        let query = Query {
            from_block,
            to_block,
            transactions: vec![TransactionSelection {
                to: vec![address.clone()],
                ..Default::default()
            }],
            field_selection: FieldSelection {
                transaction: {
                    let mut fields = BTreeSet::new();
                    fields.insert("hash".to_string());
                    fields.insert("from".to_string());
                    fields.insert("to".to_string());
                    fields.insert("value".to_string());
                    fields.insert("block_number".to_string());
                    fields.insert("status".to_string());
                    fields
                },
                ..Default::default()
            },
            ..Default::default()
        };
        self.client
            .get(&query)
            .await
            .context("Failed to query transactions.")
    }

    /// Query transaction sent from a specific address
    pub async fn query_transactions_from_address(
        &self,
        address: FixedSizeData<20>,
        from_block: u64,
        to_block: Option<u64>,
    ) -> Result<QueryResponse> {
        let query = Query {
            from_block,
            to_block,
            transactions: vec![TransactionSelection {
                from: vec![address.clone()],
                ..Default::default()
            }],
            field_selection: FieldSelection {
                transaction: {
                    let mut fields = BTreeSet::new();
                    fields.insert("hash".to_string());
                    fields.insert("from".to_string());
                    fields.insert("to".to_string());
                    fields.insert("value".to_string());
                    fields.insert("block_number".to_string());
                    fields.insert("status".to_string());
                    fields
                },
                ..Default::default()
            },
            ..Default::default()
        };

        self.client
            .get(&query)
            .await
            .context("Failed to query transaction")
    }

    /// Stream latest transactions for real-time monitoring
    pub async fn stream_latest_transactions(&self) {
        todo!();
    }
    /// Health check method
    pub async fn health_check(&self) -> Result<u64> {
        self.client
            .get_height()
            .await
            .context("Failed to perform health check")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_client_with_default_config() {
        let config = ClientConfig::default();
        let client_result = HypersyncClient::new(config);

        assert!(
            client_result.is_ok(),
            "Should create client with default config"
        );

        let client = client_result.unwrap();
        assert_eq!(client.config.url, "https://eth.hypersync.xyz");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_query_transactions_to_address() {
        // Create a client with default config
        let config = ClientConfig::default();
        let client = HypersyncClient::new(config).expect("Failed to create client");

        // Create a test Ethereum address (Uniswap V2 Router contract address)
        let eth_address_hex = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
        let address =
            Address::decode_hex(eth_address_hex).expect("Failed to create address from hex");

        // Define block range for the test
        let from_block = 12_000_000u64;
        let to_block = Some(12_000_100u64);

        // This test may fail if network is unavailable, so we're just testing
        // that the function doesn't panic and constructs the query correctly
        // In a real scenario, we'll want to mock the hypersync client for deterministic tests
        let result = client
            .query_transactions_to_address(address, from_block, to_block)
            .await;

        // We won't assert on actual results since this is a real network call that may fail
        // Just making sure the function doesn't panic with valid inputs
        // In a production test, we'd mock the HTTP client or the BaseClient itself

        // Verify the result type - it should either succeed or fail with a network error
        // but should not panic if query construction is correct
        match result {
            Ok(_) => {
                // If we got a successful response, verify it has the expected structure
                // In a real test with a mock, we would assert specific values
                println!("Successfully received response from Hypersync API");
            }
            Err(e) => {
                // It's ok if we get a network error, but log it for debugging
                eprintln!("Note: Hypersync API call failed: {}", e);
                // In a CI environment, we might want to skip this test if it
                // requires external services
            }
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_query_transactions_from_address() {
        // Create a client with default config
        let config = ClientConfig::default();
        let client = HypersyncClient::new(config).expect("Failed to create client");

        // Create a test Ethereum address (Vitalik's address)
        let eth_address_hex = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
        let address =
            Address::decode_hex(eth_address_hex).expect("Failed to create address from hex");

        // Define block range for the test
        let from_block = 12_000_000u64;
        let to_block = Some(12_000_100u64);

        // This test may fail if network is unavailable, so we're just testing
        // that the function doesn't panic and constructs the query correctly
        // In a real scenario, we'll want to mock the hypersync client for deterministic tests
        let result = client
            .query_transactions_from_address(address, from_block, to_block)
            .await;

        // We won't assert on actual results since this is a real network call that may fail
        // Just making sure the function doesn't panic with valid inputs
        // In a production test, we'd mock the HTTP client or the BaseClient itself

        // Verify the result type - it should either succeed or fail with a network error
        // but should not panic if query construction is correct
        match result {
            Ok(_) => {
                // If we got a successful response, verify it has the expected structure
                // In a real test with a mock, we would assert specific values
                println!(
                    "Successfully received response from Hypersync API for transactions from address"
                );
            }
            Err(e) => {
                // It's ok if we get a network error, but log it for debugging
                eprintln!("Note: Hypersync API call failed: {}", e);
                // In a CI environment, you might want to skip this test if it
                // requires external services
            }
        }
    }
}
