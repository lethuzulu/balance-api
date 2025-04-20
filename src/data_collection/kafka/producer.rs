// Kafka producer for transaction events

use super::config::KafkaConfig;
use crate::common::types::TransactionEvent;
use anyhow::{Context, Result};
use log::info;
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};
use std::time::Duration;

pub struct KafkaProducer {
    /// Kafka producer for message publishing
    pub producer: FutureProducer,
    /// Kafka topic for transaction events
    pub topic: String,
    /// Timeout for message publishing
    timeout: Timeout,
}

impl KafkaProducer {
    /// Create a new transaction producer with the given config
    pub fn new(config: KafkaConfig) -> Result<Self> {
        // Build the Kafka client configuration
        let mut client_config = ClientConfig::new();
        client_config
            .set("bootstrap.servers", &config.brokers)
            .set("message.timeout.ms", config.message_timeout_ms.to_string())
            .set("retry.backoff.ms", config.retry_backoff_ms.to_string())
            .set("message.send.max.retries", config.max_retries.to_string())
            .set("enable.idempotence", config.enable_idempotence.to_string());

        // Create the producer
        let producer: FutureProducer = client_config
            .create()
            .context("Failed to create Kafka producer")?;

        let timeout = Timeout::After(Duration::from_millis(config.message_timeout_ms));
        info!("Created Kafka producer connected to {}", &config.brokers);
        Ok(Self {
            producer,
            timeout,
            topic: config.default_topic,
        })
    }

    /// Send a transaction event to Kafka
    pub async fn send(&self, event: TransactionEvent) -> Result<()> {
        // Serialize the event to JSON
        let payload = serde_json::to_string(&event).context("Failed to serialize event")?;

        // Use the transaction hash as the key for Kafka partitioning
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&event.transaction_hash);

        match self.producer.send(record, self.timeout).await {
            Ok(_) => return Ok(()),
            Err((e, _)) => return Err(e.into()),
        }
    }

    /// Get a reference to the underlying producer
    pub fn get_producer(&self) -> &FutureProducer {
        &self.producer
    }
}
