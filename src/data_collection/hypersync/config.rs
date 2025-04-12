// Configuration for Hypersync

use anyhow::Result;
use hypersync_client::ClientConfig as BaseClientConfig;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub url: String,
    pub bearer_token: Option<String>,
    pub http_timeout_ms: u64,
    pub max_retries: usize,
    pub retry_backoff_ms: u64,
    pub retry_base_ms: u64,
    pub retry_ceiling_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub batch_size: u64,
    pub max_batch_size: u64,
    pub min_batch_size: u64,
    pub concurrency: usize,
    pub max_num_blocks: usize,
    pub max_num_transactions: usize,
    pub max_num_logs: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            url: "https://eth.hypersync.xyz".to_string(),
            bearer_token: None,
            http_timeout_ms: 30_000,
            max_retries: 12,
            retry_backoff_ms: 500,
            retry_base_ms: 200,
            retry_ceiling_ms: 5_000,
        }
    }
}

impl ClientConfig {
    pub fn to_base_client_config(&self) -> Result<BaseClientConfig> {
        let url = Url::parse(&self.url)?; //TODO remove unwrap()
        let http_timeout = NonZeroU64::new(self.http_timeout_ms).unwrap(); //TODO remove unwrap()
        Ok(BaseClientConfig {
            url: Some(url),
            bearer_token: self.bearer_token.clone(),
            http_req_timeout_millis: Some(http_timeout),
            max_num_retries: Some(self.max_retries),
            retry_backoff_ms: Some(self.retry_backoff_ms),
            retry_base_ms: Some(self.retry_backoff_ms),
            retry_ceiling_ms: Some(self.retry_ceiling_ms),
        })
    }
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            max_batch_size: 5000,
            min_batch_size: 100,
            concurrency: 4, //todo! consider making equal to the number of available cores
            max_num_blocks: 1000,
            max_num_transactions: 10000,
            max_num_logs: 10000,
        }
    }
}

/// Load Hyoersync configuration from environment variables
pub fn load_config() -> Result<ClientConfig> {
    // Expand to laod from env file
    let mut config = ClientConfig::default();

    if let Ok(endpoint) = std::env::var("HYPERSYNC_ENDPOINT") {
        config.url = endpoint;
    }
    // Additional environment variable parsing could be added here
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_client_config() {
        let config = ClientConfig::default();
        assert_eq!(config.url, "https://eth.hypersync.xyz");
        assert_eq!(config.bearer_token, None);
        assert_eq!(config.http_timeout_ms, 30_000);
        assert_eq!(config.max_retries, 12);
        assert_eq!(config.retry_backoff_ms, 500);
        assert_eq!(config.retry_base_ms, 200);
        assert_eq!(config.retry_ceiling_ms, 5_000);
    }

    #[test]
    fn test_to_base_client_config() {
        let config = ClientConfig::default();
        let base_config = config.to_base_client_config().unwrap();
        assert_eq!(
            base_config.url.unwrap().as_str(),
            "https://eth.hypersync.xyz/"
        );
    }
}
