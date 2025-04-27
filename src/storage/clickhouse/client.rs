// ClickHouse client
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use clickhouse::Client;

use crate::storage::models::{
    balance::{BalanceChange, CurrentBalance, HistoricalBalance},
    transaction::TransactionRecord,
};

pub struct ClickhouseClient {
    client: Client,
}

impl ClickhouseClient {
    pub fn new() -> Self {
        let client = Client::default()
            .with_url("http://localhost:8123")
            .with_database("default");
        ClickhouseClient { client }
    }

    pub async fn insert_transaction(&self, balance: &TransactionRecord) -> Result<()> {
        let q = "
        INSERT INTO transaction_records (block_number, transaction_hash, from_address, to_address, value, timestamp, chain_id, is_sucess)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?) 
        ";
        self.client
            .query(q)
            .bind(balance.block_number)
            .bind(&balance.transaction_hash)
            .bind(&balance.from_address)
            .bind(&balance.to_address)
            .bind(&balance.value)
            .bind(&balance.timestamp)
            .bind(&balance.chain_id)
            .bind(balance.is_success)
            .execute()
            .await
            .context("Failed to insert transaction")?;
        Ok(())
    }

    pub async fn insert_balance_change(&self, balance_change: &BalanceChange) -> Result<()> {
        let q = "
        INSERT INTO balance_changes (address, block_number, change_amount, transaction_hash, timestamp, chain_id)
        VALUES (?, ?, ?, ?, ?, ?)
        ";
        let _insert_result = self
            .client
            .query(q)
            .bind(&balance_change.address)
            .bind(balance_change.block_number)
            .bind(&balance_change.change_amount)
            .bind(&balance_change.transaction_hash)
            .bind(balance_change.timestamp)
            .bind(balance_change.chain_id)
            .execute()
            .await?;

        Ok(())
    }

    pub async fn insert_historical_balance(&self, balance: &HistoricalBalance) -> Result<()> {
        let q = "
        INSERT INTO historical_balances (address, block_number, balance, timestamp, chain_id)
        VALUES (?, ?, ?, ?, ?)
        ";
        let _result = self
            .client
            .query(q)
            .bind(&balance.address)
            .bind(balance.block_number)
            .bind(&balance.balance)
            .bind(balance.timestamp)
            .bind(balance.chain_id)
            .execute()
            .await?;

        Ok(())
    }

    pub async fn get_current_balance(
        &self,
        address: &str,
        chain_id: u64,
    ) -> Result<CurrentBalance> {
        let q = "
            SELECT address, chain_id, balance, last_block_number, last_update 
            FROM current_balances
            WHERE address = ? AND chain_id = ?";

        let result = self
            .client
            .query(q)
            .bind(address)
            .bind(chain_id)
            .fetch_optional::<CurrentBalance>()
            .await;

        match result {
            Ok(Some(balance)) => Ok(balance),
            Ok(None) => {
                // Return a default balance of zero if no record exists
                Ok(CurrentBalance {
                    address: address.to_string(),
                    chain_id,
                    balance: "0".to_string(),
                    last_block_number: 0,
                    last_update: Utc::now(),
                })
            }
            Err(e) => Err(anyhow!("Failed to get current balance: {}", e)),
        }
    }

    pub async fn get_balance_at_block(
        &self,
        address: &str,
        block_number: u64,
        chain_id: u64,
    ) -> Result<String> {
        // Query for the test balance before or at the specifie block
        let query = "
        SELECT balance 
        FROM historical_balances
        WHERE address = ? AND chan_id = ? AND block_number <= ?
        ORDER BY block_number DESC
        LIMIT 1
        ";
        let result = self
            .client
            .query(query)
            .bind(address)
            .bind(block_number)
            .bind(chain_id)
            .fetch_optional::<HistoricalBalance>()
            .await;

        match result {
            Ok(Some(balance)) => Ok(balance.balance),
            Ok(None) => Ok("0".into()),
            Err(e) => Err(anyhow!("Failed to get balance: {}", e)),
        }
    }
}
