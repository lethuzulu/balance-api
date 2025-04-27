use crate::storage::models::balance::{BalanceChange as DbBalanceChange, HistoricalBalance};
use crate::{
    common::types::TransactionEvent,
    processing::models::balance::BalanceChange,
    storage::{clickhouse::client::ClickhouseClient, models::transaction::TransactionRecord},
};
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;
use log::info;


pub struct TransactionProcessor {
    //config: ProcessorConfig,  //TODO: add a config
    db_client: ClickhouseClient,
    latest_blocks: HashMap<u64, u64>,
}

impl TransactionProcessor {
    pub fn new() -> Result<Self> {
        let db_client = ClickhouseClient::new();
        Ok(Self {
            db_client,
            latest_blocks: HashMap::new(),
        })
    }

    pub async fn start(self, mut receiver: Receiver<TransactionEvent>) {
        info!(" Starting transaction processor");
        
        // Spawn a task to process received messages
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                if let Err(e) = self.process_transaction(&event).await {
                    log::error!("Error process transaction {}: {}", event.transaction_hash, e);
                }
            }
        });
    }

    pub async fn process_transaction(&self, event: &TransactionEvent) -> Result<()> {
        
        let mut balance_changes = Vec::new();

        // Convert event to transaction record
        let transaction = TransactionRecord::from(event.clone());

        // Store the transaction
        self.db_client
            .insert_transaction(&transaction)
            .await
            .context("Failed to insert transaction record")?;

        let timestamp = DateTime::from_timestamp(event.timestamp as i64, 0).unwrap_or_default();
        // For sender: this is a negative balance change (i.e. outgoing)
        if !event.value.is_empty() && event.value != "0" && event.value != "0x0" {
            let value = if event.value.starts_with("0x") {
                // convert hex to decimal
                let value_str = &event.value[2..];
                let value_decimal = u128::from_str_radix(value_str, 16)
                    .map_err(|e| anyhow!("Failed to parse hex value: {}", e))?;
                value_decimal.to_string()
            } else {
                event.value.clone()
            };

            // Sender balance decreases
            let sender_change = BalanceChange {
                address: event.from_address.clone(),
                block_number: event.block_number,
                amount: format!("-{}", value), // Negative for outgoing
                transaction_hash: event.transaction_hash.clone(),
                timestamp: DateTime::from_timestamp(event.timestamp as i64, 0).unwrap_or_default(),
                chain_id: event.chain_id,
            };
            balance_changes.push(sender_change.clone());

            // Convert to DB model and store
            let db_sender_change = DbBalanceChange {
                address: sender_change.address.clone(),
                block_number: sender_change.block_number,
                change_amount: sender_change.amount.clone(),
                transaction_hash: sender_change.transaction_hash.clone(),
                timestamp: sender_change.timestamp,
                chain_id: sender_change.chain_id,
            };
            self.db_client
                .insert_balance_change(&db_sender_change)
                .await?;

            // Receiver balance increases
            let receiver_change = BalanceChange {
                address: event.to_address.clone(),
                block_number: event.block_number,
                amount: value, // Positive for incoming
                transaction_hash: event.transaction_hash.clone(),
                timestamp: DateTime::from_timestamp(event.timestamp as i64, 0).unwrap_or_default(),
                chain_id: event.chain_id,
            };
            balance_changes.push(receiver_change.clone());

            // Convert to DB model and store
            let db_receiver_change = DbBalanceChange {
                address: receiver_change.address.clone(),
                block_number: receiver_change.block_number,
                change_amount: receiver_change.amount.clone(),
                transaction_hash: receiver_change.transaction_hash.clone(),
                timestamp: receiver_change.timestamp,
                chain_id: receiver_change.chain_id,
            };
            self.db_client
                .insert_balance_change(&db_receiver_change)
                .await?;

            // // Update historical balances - Purpose: Fast lookup for balance at specific blocks
            self.update_historical_balance(&event.from_address, event.block_number, event.chain_id, timestamp).await?;
            self.update_historical_balance(&event.to_address, event.block_number, event.chain_id, timestamp).await?;
        }
        // Update latest processed block for this chain
        self.update_latest_block(event.chain_id, event.block_number)
            .await?;
        Ok(())
    }

    async fn update_historical_balance(
        &self,
        address: &str,
        block_number: u64,
        chain_id: u64,
        timestamp: DateTime<Utc>,
    ) -> Result<()> {
        // Get current_balance
        let balance = self.db_client.get_current_balance(address, block_number).await?;

        // Create historical balance entry
        let historical_balance = HistoricalBalance{
            address: address.to_string(),
            block_number,
            balance: balance.balance,
            timestamp,
            chain_id
        };

        // Store historical balance
        self.db_client.insert_historical_balance(&historical_balance).await?;

        Ok(())
    }

    async fn update_latest_block(&self, chain_id: u64, block_number: u64) -> Result<()> {
        let mut latest_blocks = self.latest_blocks.clone();

        if let Some(latest) = latest_blocks.get(&chain_id) {
            if block_number > *latest {
                latest_blocks.insert(chain_id, block_number);
            }
        }
        Ok(())
    }
}
