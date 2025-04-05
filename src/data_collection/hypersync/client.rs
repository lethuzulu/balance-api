// Hypersync client wrapper

use hypersync_client::Client as BaseClient;
use anyhow::{Result, Context};

use super::config::ClientConfig;


pub struct HyperSyncClient {
    client: BaseClient,
    config: ClientConfig
}


impl HyperSyncClient {
    pub fn new(config: ClientConfig) -> Result<Self> {
        let base_config = config.to_base_client_config().context("Failed to create client config")?;
        let base_client = BaseClient::new(base_config).context("Failed to create HyperSync Client")?;

        Ok(HyperSyncClient { 
            client: base_client,
            config
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_client_with_default_config() {
        let config = ClientConfig::default();
        let client_result = HyperSyncClient::new(config);
        
        assert!(client_result.is_ok(), "Should create client with default config");
        
        let client = client_result.unwrap();
        assert_eq!(client.config.url, "https://eth.hypersync.xyz");
    }
}