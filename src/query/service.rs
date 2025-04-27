// Query service
use anyhow::{Context, Result};

use crate::storage::{clickhouse::client::ClickhouseClient, models::balance::CurrentBalance};

pub struct QueryService {
    db_client: ClickhouseClient,
}

impl QueryService {
    pub fn new() -> Result<Self> {
        let db_client = ClickhouseClient::new();
        Ok(Self { db_client })
    }

    pub async fn get_balance_at_block(
        &self,
        address: &str,
        block_number: u64,
        chain_id: u64,
    ) -> Result<String> {
        self.db_client
            .get_balance_at_block(address, block_number, chain_id)
            .await
    }

    pub async fn get_current_balance(
        &self,
        address: &str,
        chain_id: u64,
    ) -> Result<CurrentBalance> {
        self.db_client.get_current_balance(address, chain_id).await
    }
}
