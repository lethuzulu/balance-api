#[derive(Debug, Clone)]
pub struct KafkaConfig {
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
    pub enable_idempotence: bool,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            brokers: "localhost:9092".into(),
            default_topic: "raw.transactions".into(),
            message_timeout_ms: 5000,
            retry_backoff_ms: 500,
            max_retries: 5,
            enable_idempotence: true,
        }
    }
}

/// Load Kafka configuration from environment variables
pub fn load_kafka_config() -> KafkaConfig {
    let mut config = KafkaConfig::default();

    if let Ok(brokers) = std::env::var("KAFKA_BROKERS") {
        config.brokers = brokers;
    }
    // Additional environment variable parsing to be added
    config
}
