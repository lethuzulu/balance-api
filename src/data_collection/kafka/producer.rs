// Kafka producer for transaction events
// helper methods for proper error handling and exactly-once delivery semantics
use anyhow::{Context, Result};
use log::{debug, error, info};
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    message::ToBytes,
};
use serde::Serialize;
use std::time::Duration;
use std::future::Future;
use std::pin::Pin;

pub struct ProducerConfig {
    /// List of Kafka bootstrap servers (comma-separated)
    pub brokers: String,
    /// Default topic to publish messages to
    pub default_topic: String,
    /// Message timeout in milliseconds
    pub message_timeout_ms: u64,
    /// Retry backoff in milliseconds
    pub retry_backoff_ms: u64,
    /// Maximum number of retries
    pub max_retries: u64,
    /// Whether to enable idempotent delivery
    pub enable_idempotence: bool
}

impl Default for ProducerConfig {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".to_string(),
            default_topic: "transactions".to_string(),
            message_timeout_ms: 5000,
            retry_backoff_ms: 500,
            max_retries: 5,
            enable_idempotence: true,
        }
    }
}

pub struct TransactionProducer {
    /// Kafka producer for message publishing
    producer: FutureProducer,
    /// Default topic for messages
    default_topic: String,
    /// Timeout for message publishing
    timeout: Duration,
}

impl TransactionProducer {

    /// Create a new transaction producer with the given config
    pub fn new(config: ProducerConfig) -> Result<Self> {
        // Build the Kafka client configuration
        let mut client_config = ClientConfig::new();
        client_config
        .set("bootstrap.servers", &config.brokers)
        .set("message.timeout.ms", &config.message_timeout_ms.to_string())
        .set("retry.backoff.ms", &config.retry_backoff_ms.to_string())
        .set("message.send.max.retries", &config.max_retries.to_string());
        // Enable idempotent delivery if specified
        if config.enable_idempotence {
            client_config.set("enable.idempotence", "true");
        }
        // Create the producer
        let producer: FutureProducer = client_config.create().context("Failed to create kafka producer")?;
        info!("Created Kafka producer connected to {}", config.brokers);
        Ok (Self {
            producer,
            default_topic: config.default_topic,
            timeout: Duration::from_millis(config.message_timeout_ms)
        })
    }

    /// Send a message to Kafka with the given key and value
    pub async fn send<K, V>(&self, key: K, value: &V, topic: Option<String>) -> Result<()>
    where
        K: AsRef<str>,
        V: Serialize + ?Sized,
    {
        let topic = topic.unwrap_or_else(|| self.default_topic.clone());
        
        // Serialize the value to JSON
        let payload = serde_json::to_string(value)
            .context("Failed to serialize value to JSON")?;
        
        // Create the Kafka record
        let record = FutureRecord::to(&topic)
            .payload(&payload)
            .key(key.as_ref());
        
        debug!("Sending message to topic {}", topic);
        
        // Send the record with timeout
        match self.producer.send(record, Timeout::After(self.timeout)).await {
            Ok(_) => {
                debug!("Successfully sent message to Kafka");
                Ok(())
            },
            Err((err, _)) => {
                error!("Failed to send message to Kafka: {}", err);
                Err(anyhow::anyhow!("Failed to send message to Kafka: {}", err))
            }
        }
    }
    
    /// Send a batch of messages to Kafka with the given key and values
    pub async fn send_batch<K, V>(&self, messages: Vec<(K, V)>, topic: Option<String>) -> Result<usize>
    where
        K: AsRef<str>,
        V: Serialize,
    {
        let mut successful_count = 0;
        let target_topic = topic.unwrap_or_else(|| self.default_topic.clone());
        let total_messages = messages.len();
        
        for (key, value) in messages {
            match self.send(key, &value, Some(target_topic.clone())).await {
                Ok(_) => successful_count += 1,
                Err(e) => error!("Error sending message: {}", e),
            }
        }
        
        info!("Successfully sent {}/{} messages", successful_count, total_messages);
        Ok(successful_count)
    }
    
    /// Get a reference to the underlying producer
    pub fn get_producer(&self) -> &FutureProducer {
        &self.producer
    }

}