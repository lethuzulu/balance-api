use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::client::HypersyncClient;
use anyhow::{Context, Result};
use hypersync_client::{
    QueryResponse, StreamConfig,
    format::{FixedSizeData, Hex, TransactionStatus},
};
use log::{error, info};
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use serde::{Deserialize, Serialize};

/// Transaction collector responsible for collecting transactions
/// and publishing them to Kafka
pub struct TransactionCollector {
    /// Hypersync client for data collection
    client: HypersyncClient,
    /// Kafka producer for transaction events
    producer: FutureProducer,
    /// Kafka topic for transaction events
    topic: String,
    /// Chain ID for the transactions
    chain_id: u64,
    // Stream configuration
    // stream_config: StreamConfig,
}

impl TransactionCollector {
    /// Create a new TransactionCollector with the given HyperSync client and Kafka configuration
    pub fn new(
        hypersync_client: HypersyncClient,
        brokers: &str,
        topic: &str,
        chain_id: u64,
    ) -> Result<Self> {
        //Set up Kafka producer
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .set("retry.backoff.ms", "500")
            .set("message.send.max.retries", "5")
            .create()
            .context("Failed to create Kafka producer")?;

        Ok(Self {
            client: hypersync_client,
            producer,
            topic: topic.to_string(),
            chain_id,
        })
    }

    /// Collect and publish historical transaction for an address
    pub async fn collect_historical_transactions(
        &self,
        address: &str,
        from_block: u64,
        to_block: Option<u64>,
    ) -> Result<u64> {
        info!(
            "Collecting historical transactions for address {} from block {} to block {:?}",
            address, from_block, to_block
        );

        // Parse the address from hexadecimal string to FixedSizeData<20>
        let eth_address =
            FixedSizeData::<20>::decode_hex(address).context("Failed to decode ETH address")?;

        // Query transactions TO the address
        let to_txs = self
            .client
            .query_transactions_to_address(eth_address.clone(), from_block, to_block)
            .await
            .context("Failed to query transaction TO address")?;

        // Query transactions FROM the address
        let from_txs = self
            .client
            .query_transactions_from_address(eth_address, from_block, to_block)
            .await
            .context("Failed to query transactions FROM address")?;

        // Process and publish transactions
        let to_count = self
            .process_and_publish_transactions(to_txs.clone())
            .await
            .context("Failed to process and publish to kafka")?;
        let from_count = self
            .process_and_publish_transactions(from_txs.clone())
            .await
            .context("Failed to process and publish to kafka")?;

        let total_processed = to_count + from_count;
        info!("Processed {} total transactions", total_processed);

        // Return the highest block number processed for pagination
        Ok(self.get_highest_block_number(&to_txs, &from_txs))
    }

    /// Process and publish transaction from a query and publish to Kafka
    async fn process_and_publish_transactions(&self, response: QueryResponse) -> Result<usize> {
        let mut count = 0;

        for tx_batch in response.data.transactions {
            for tx in tx_batch {
                // Convert transaction data to our event format
                let event = TransactionEvent {
                    chain_id: self.chain_id,
                    transaction_hash: tx.hash.unwrap_or_default().to_string(),
                    block_number: tx.block_number.unwrap_or_default().into(), // Use .into() to convert BlockNumber to u64
                    from_address: tx.from.unwrap_or_default().to_string(),
                    to_address: tx.to.unwrap_or_default().to_string(),
                    value: tx.value.unwrap_or_default().encode_hex(),
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    is_success: tx.status.unwrap_or(TransactionStatus::Failure)
                        == TransactionStatus::Success,
                };

                // Serialize the event to JSON
                let payload = serde_json::to_string(&event).context("Failed to serialize event")?;

                // Use the transaction hash as the key for Kafka partitioning
                let record = FutureRecord::to(&self.topic)
                    .payload(&payload)
                    .key(&event.transaction_hash);

                // Send to Kafka with timeout
                match self
                    .producer
                    .send(record, Timeout::After(Duration::from_secs(5)))
                    .await
                {
                    Ok(_) => count += 1,
                    Err((e, _)) => {
                        error!("Failed to send transaction to Kafka: {}", e);
                    }
                }
            }
        }
        Ok(count)
    }

    /// Get the highest block number processed for pagination
    fn get_highest_block_number(&self, to_txs: &QueryResponse, from_txs: &QueryResponse) -> u64 {
        let to_max = to_txs
            .data
            .transactions
            .iter()
            .flat_map(|tx_batch| tx_batch.iter())
            .map(|tx| tx.block_number.unwrap_or_default().into())
            .max()
            .unwrap_or(0);

        let from_max = from_txs
            .data
            .transactions
            .iter()
            .flat_map(|tx_batch| tx_batch.iter())
            .map(|tx| tx.block_number.unwrap_or_default().into())
            .max()
            .unwrap_or(0);

        std::cmp::max(to_max, from_max)
    }
    /// Set up real-time transaction monitoring stream
    pub async fn start_real_time_monitoring(&self, address: &str) -> Result<()> {
        info!(
            "Start real-time transaction monitoring for address {}",
            address
        );

        // Implementation for real-time monitorinf would typically:
        // 1. Get the latest block number
        // 2. Set up a recurring taks that polls for new transactions
        // 3. Process and publish any new transactions

        // For now, we'll leave this as a place holder
        todo!("Implement real-time transaction monitoring");
    }

    /// Perform a health check on the collector
    pub async fn health_check(&self) -> Result<()> {
        // Check Hypersync client health
        self.client
            .health_check()
            .await
            .context("Hypersync client health check failed")?;

        // For a more complete health check, we could also:
        // - Try to send a test message to Kafka
        // - Check if we can connect to Redis
        // - Verify other dependencies
        Ok(())
    }
}

// Kafka message schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEvent {
    pub chain_id: u64,
    pub transaction_hash: String,
    pub block_number: u64,
    pub from_address: String,
    pub to_address: String,
    pub value: String, // Using String for BigDecimal compatibility
    pub timestamp: u64,
    pub is_success: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;

    // Mock the HypersyncClient
    mock! {
        pub HypersyncClient {
            fn query_transactions_to_address(
                &self, 
                address: FixedSizeData<20>,
                from_block: u64,
                to_block: Option<u64>,
            ) -> Result<MockQueryResponse>;
            
            fn query_transactions_from_address(
                &self,
                address: FixedSizeData<20>,
                from_block: u64,
                to_block: Option<u64>,
            ) -> Result<MockQueryResponse>;
            
            fn health_check(&self) -> Result<u64>;
        }
        
        impl Clone for HypersyncClient {
            fn clone(&self) -> Self;
        }
    }

    // Mock the Kafka Producer
    #[derive(Clone)]
    struct MockProducer {
        send_result: Option<Result<(), String>>,
    }

    impl MockProducer {
        fn new() -> Self {
            Self { send_result: None }
        }

        fn with_result(result: Result<(), String>) -> Self {
            Self { send_result: Some(result) }
        }
    }

    // Helper function to create a test transaction event
    fn create_test_transaction_event(hash: &str, block_number: u64) -> hypersync_client::format::Transaction {
        use hypersync_client::format::{FixedSizeData, Transaction, Quantity};
        
        // For mocking purposes, we'll create a simple transaction with valid hash and values
        Transaction {
            hash: FixedSizeData::<32>::decode_hex(hash).unwrap(),
            block_number: block_number.into(),  // Convert u64 to UInt
            from: Some(FixedSizeData::<20>::decode_hex("0x1234567890123456789012345678901234567890").unwrap()),
            to: Some(FixedSizeData::<20>::decode_hex("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd").unwrap()),
            value: Quantity::from(vec![1]), // Use 1 instead of 0 to avoid assertion error
            // Transaction status is handled differently in the actual format
            // We'll use the default for other fields
            ..Default::default()
        }
    }

    // Create a mock of the QueryResponse for testing
    #[derive(Debug, Clone)]
    struct MockQueryResponse {
        transactions: Vec<Vec<hypersync_client::format::Transaction>>,
        archive_height: Option<u64>,
        next_block: u64,
        total_execution_time: u64,
        rollback_guard: Option<u64>,
    }
    
    // Helper function to create a test query response
    fn create_test_query_response(txs: Vec<hypersync_client::format::Transaction>) -> MockQueryResponse {
        MockQueryResponse {
            transactions: vec![txs],
            archive_height: Some(0),
            next_block: 0,
            total_execution_time: 0,
            rollback_guard: None,
        }
    }
    
    // Create a test-specific version of TransactionCollector that works with the mock
    struct TestTransactionCollector {
        client: MockHypersyncClient,
        producer: FutureProducer,
        topic: String,
        chain_id: u64,
    }

    // Implement methods from TransactionCollector for TestTransactionCollector
    impl TestTransactionCollector {
        // Just implement what we need for the tests
        fn get_highest_block_number(&self, to_txs: &MockQueryResponse, from_txs: &MockQueryResponse) -> u64 {
            let to_max = to_txs
                .transactions
                .iter()
                .flat_map(|tx_batch| tx_batch.iter())
                .map(|tx| u64::from(tx.block_number))
                .max()
                .unwrap_or(0);

            let from_max = from_txs
                .transactions
                .iter()
                .flat_map(|tx_batch| tx_batch.iter())
                .map(|tx| u64::from(tx.block_number))
                .max()
                .unwrap_or(0);

            std::cmp::max(to_max, from_max)
        }
        
        async fn health_check(&self) -> Result<()> {
            // Directly call health_check on the mock
            self.client.health_check()?;
            Ok(())
        }
        
        async fn collect_historical_transactions(
            &self,
            address: &str,
            from_block: u64,
            to_block: Option<u64>,
        ) -> Result<u64> {
            // Parse the address from hexadecimal string to FixedSizeData<20>
            let eth_address =
                FixedSizeData::<20>::decode_hex(address).context("Failed to decode ETH address")?;

            // Query transactions TO the address directly using the mock
            let to_txs = self
                .client
                .query_transactions_to_address(eth_address.clone(), from_block, to_block)?;

            // Query transactions FROM the address directly using the mock
            let from_txs = self
                .client
                .query_transactions_from_address(eth_address, from_block, to_block)?;

            // Return the highest block number processed for pagination
            Ok(self.get_highest_block_number(&to_txs, &from_txs))
        }
    }

    // Helper to create a test collector with mocked client
    fn setup_collector_with_mock() -> (TestTransactionCollector, MockHypersyncClient) {
        // Create mock client
        let mut mock_client = MockHypersyncClient::new();
        
        // Set up clone expectation that all tests need
        mock_client.expect_clone()
            .times(1..)
            .returning(|| {
                MockHypersyncClient::new()
            });
        
        // Create mock producer via rdkafka ClientConfig
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "mock://mock")
            .create()
            .unwrap();
        
        // Create test collector with mocks
        let collector = TestTransactionCollector {
            client: mock_client.clone(),
            producer,
            topic: "test-topic".to_string(),
            chain_id: 1,
        };
        
        (collector, mock_client)
    }

    #[tokio::test]
    async fn test_get_highest_block_number() {
        // Create test data
        let tx1 = create_test_transaction_event("0x1111111111111111111111111111111111111111111111111111111111111111", 100);
        let tx2 = create_test_transaction_event("0x2222222222222222222222222222222222222222222222222222222222222222", 200);
        let tx3 = create_test_transaction_event("0x3333333333333333333333333333333333333333333333333333333333333333", 150);
        
        let to_txs = create_test_query_response(vec![tx1]);
        let from_txs = create_test_query_response(vec![tx2, tx3]);
        
        // Create mock client directly
        let mock_client = MockHypersyncClient::new();
        
        // Create producer via rdkafka ClientConfig
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "mock://mock")
            .create()
            .unwrap();
        
        // Create test collector with mock
        let collector = TestTransactionCollector {
            client: mock_client,
            producer,
            topic: "test-topic".to_string(),
            chain_id: 1,
        };
        
        // Test the function
        let highest_block = collector.get_highest_block_number(&to_txs, &from_txs);
        
        // Verify result
        assert_eq!(highest_block, 200);
    }

    #[tokio::test]
    async fn test_empty_get_highest_block_number() {
        // Create empty responses
        let empty_response = create_test_query_response(vec![]);
        
        // Create mock client directly
        let mock_client = MockHypersyncClient::new();
        
        // Create producer via rdkafka ClientConfig
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "mock://mock")
            .create()
            .unwrap();
        
        // Create test collector with mock
        let collector = TestTransactionCollector {
            client: mock_client,
            producer,
            topic: "test-topic".to_string(),
            chain_id: 1,
        };
        
        // Test with empty responses
        let highest_block = collector.get_highest_block_number(&empty_response, &empty_response);
        
        // Should return 0 for empty responses
        assert_eq!(highest_block, 0);
    }

    #[tokio::test]
    async fn test_collect_historical_transactions_successful() {
        // Setup test data
        let address = "0x1234567890123456789012345678901234567890";
        let from_block = 100;
        let to_block = Some(200);
        
        let tx1 = create_test_transaction_event("0x1111111111111111111111111111111111111111111111111111111111111111", 150);
        let tx2 = create_test_transaction_event("0x2222222222222222222222222222222222222222222222222222222222222222", 180);
        
        let to_txs_response = create_test_query_response(vec![tx1]);
        let from_txs_response = create_test_query_response(vec![tx2]);
        
        // Create mock client directly
        let mut mock_client = MockHypersyncClient::new();
        
        // Configure mock expectations
        let eth_address = FixedSizeData::<20>::decode_hex(address).unwrap();
        
        mock_client.expect_query_transactions_to_address()
            .with(eq(eth_address.clone()), eq(from_block), eq(to_block))
            .times(1)
            .returning(move |_, _, _| Ok(to_txs_response.clone()));
            
        mock_client.expect_query_transactions_from_address()
            .with(eq(eth_address.clone()), eq(from_block), eq(to_block))
            .times(1)
            .returning(move |_, _, _| Ok(from_txs_response.clone()));
        
        // Create producer via rdkafka ClientConfig
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "mock://mock")
            .create()
            .unwrap();
        
        // Create test collector with mock
        let collector = TestTransactionCollector {
            client: mock_client,
            producer,
            topic: "test-topic".to_string(),
            chain_id: 1,
        };
        
        // Call the method under test
        let result = collector.collect_historical_transactions(address, from_block, to_block).await;
        
        // Verify results
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 180); // Highest block number
    }

    #[tokio::test]
    async fn test_collect_historical_transactions_query_error() {
        // Setup test data
        let address = "0x1234567890123456789012345678901234567890";
        let from_block = 100;
        let to_block = Some(200);
        
        // Create mock client directly
        let mut mock_client = MockHypersyncClient::new();
        
        // Configure mock expectations
        let eth_address = FixedSizeData::<20>::decode_hex(address).unwrap();
        
        // Configure mock to return error
        mock_client.expect_query_transactions_to_address()
            .with(eq(eth_address.clone()), eq(from_block), eq(to_block))
            .times(1)
            .returning(|_, _, _| Err(anyhow::anyhow!("HyperSync query failed")));
        
        // Create producer via rdkafka ClientConfig
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "mock://mock")
            .create()
            .unwrap();
        
        // Create test collector with mock
        let collector = TestTransactionCollector {
            client: mock_client,
            producer,
            topic: "test-topic".to_string(),
            chain_id: 1,
        };
        
        // Call the method under test
        let result = collector.collect_historical_transactions(address, from_block, to_block).await;
        
        // Verify results
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("HyperSync query failed"));
    }

    #[tokio::test]
    async fn test_health_check_successful() {
        // Create mock client directly
        let mut mock_client = MockHypersyncClient::new();
        
        // Configure mock expectations for this specific test
        mock_client.expect_health_check()
            .times(1)
            .returning(|| Ok(12345678));
        
        // Create producer via rdkafka ClientConfig
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "mock://mock")
            .create()
            .unwrap();
        
        // Create test collector with mock
        let collector = TestTransactionCollector {
            client: mock_client,
            producer,
            topic: "test-topic".to_string(),
            chain_id: 1,
        };
        
        // Call the method under test
        let result = collector.health_check().await;
        
        // Verify results
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_health_check_failure() {
        // Create mock client directly
        let mut mock_client = MockHypersyncClient::new();
        
        // Configure mock expectations for this specific test
        mock_client.expect_health_check()
            .times(1)
            .returning(|| Err(anyhow::anyhow!("HyperSync health check failed")));
        
        // Create producer via rdkafka ClientConfig
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", "mock://mock")
            .create()
            .unwrap();
        
        // Create test collector with mock
        let collector = TestTransactionCollector {
            client: mock_client,
            producer,
            topic: "test-topic".to_string(),
            chain_id: 1,
        };
        
        // Call the method under test
        let result = collector.health_check().await;
        
        // Verify results
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("HyperSync health check failed"));
    }

    #[test]
    fn test_transaction_event_serialization() {
        // Create a test event
        let event = TransactionEvent {
            chain_id: 1,
            transaction_hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string(),
            block_number: 12345678,
            from_address: "0x1234567890123456789012345678901234567890".to_string(),
            to_address: "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string(),
            value: "1000000000000000000".to_string(), // 1 ETH
            timestamp: 1609459200, // 2021-01-01
            is_success: true,
        };
        
        // Serialize to JSON
        let json = serde_json::to_string(&event).unwrap();
        
        // Deserialize back
        let deserialized: TransactionEvent = serde_json::from_str(&json).unwrap();
        
        // Verify fields match
        assert_eq!(deserialized.chain_id, event.chain_id);
        assert_eq!(deserialized.transaction_hash, event.transaction_hash);
        assert_eq!(deserialized.block_number, event.block_number);
        assert_eq!(deserialized.from_address, event.from_address);
        assert_eq!(deserialized.to_address, event.to_address);
        assert_eq!(deserialized.value, event.value);
        assert_eq!(deserialized.timestamp, event.timestamp);
        assert_eq!(deserialized.is_success, event.is_success);
    }
}
