// Query service
use anyhow::{Context, Result};

use crate::storage::clickhouse::client::ClickhouseClient;

pub struct QueryService {
    db_client: ClickhouseClient,
}

impl QueryService {
    pub fn new() -> Result<Self> {
        // Construct DB connection string from environment variables or config
        let connection_string =
            std::env::var("CLICKHOUSE_URL").unwrap_or_else(|_| "tcp://localhost:9000".into());

        let db_client = ClickhouseClient::new();

        Ok(Self { db_client })
    }

    pub async fn get_balance_at_block(
        &self,
        address: &str,
        block_number: u64,
        chain_id: u64,
    ) -> Result<()> {
        let _result = self
            .db_client
            .get_balance_at_block(address, block_number, chain_id)
            .await;
        Ok(())
    }

    // pub async fn get_current_balance(&self, address: &str, chain_id: u64) -> Result<()> {
    //     // self.db
    //     Ok(())
    // }
}
