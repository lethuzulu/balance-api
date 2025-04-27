// GraphQL schema
use anyhow::Result;
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use chrono::{DateTime, Utc};

use crate::query::service::QueryService;

#[derive(Debug, SimpleObject)]
struct Balance {
    address: String,
    chain_id: u64,
    balance: String,
    block_number: u64,
    timestamp: DateTime<Utc>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn balance_at_block(
        &self,
        ctx: &Context<'_>,
        address: String,
        block_number: u64,
        chain_id: u64,
    ) -> Result<Balance> {
        let service = ctx.data::<QueryService>().unwrap(); // TODO remove unwrap
        let balance = service
            .get_balance_at_block(&address, block_number, chain_id)
            .await?;

        let timestamp = Utc::now(); // TODO: fetch the block's real timestamp
        Ok(Balance {
            address,
            chain_id,
            balance,
            block_number,
            timestamp,
        })
    }

    async fn current_balance(
        &self,
        ctx: &Context<'_>,
        address: String,
        chain_id: i64,
    ) -> Result<Balance> {
        let service = ctx.data::<QueryService>().unwrap();
        let balance = service
            .get_current_balance(&address, chain_id as u64)
            .await?;

        Ok(Balance {
            address: balance.address,
            chain_id: balance.chain_id,
            balance: balance.balance,
            block_number: balance.last_block_number,
            timestamp: balance.last_update,
        })
    }
}

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema(service: QueryService) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(service)
        .finish()
}
