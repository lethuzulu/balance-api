// GraphQL schema
use crate::data_collection::collection_manager::{self, CollectionManager, CollectionStatus};
use crate::query::service::QueryService;
use anyhow::{Result, anyhow};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};
use chrono::{DateTime, Utc};
use std::sync::Arc;

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
        let collection_manager = ctx.data::<Arc<CollectionManager>>().unwrap(); //TODO: remove unwrap
        let status = collection_manager
            .get_or_trigger_collection(address.clone(), block_number)
            .await?;

        match status {
            CollectionStatus::Ready => {
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
            CollectionStatus::Collecting | CollectionStatus::NotStarted => Err(anyhow!(
                "Data for this address is being collected. Please try again soon."
            )),
            CollectionStatus::Error(e) => Err(anyhow!("Collection failed: {}", e)),
        }
    }
}

// async fn current_balance(
//     &self,
//     ctx: &Context<'_>,
//     address: String,
//     chain_id: i64,
// ) -> Result<Balance> {
//     let collection_manager = ctx.data::<Arc<CollectionManager>>().unwrap(); // remove unwrap
//     let status = collection_manager.get_or_trigger_collection(address.clone()).await;

//     match status {
//         CollectionStatus::Ready => {
//             let service = ctx.data::<QueryService>().unwrap();
//             let balance = service.get_current_balance(&address, chain_id as u64).await?;
//             Ok(Balance {
//                 address: balance.address,
//                 chain_id: balance.chain_id,
//                 balance: balance.balance,
//                 block_number: balance.last_block_number,
//                 timestamp: balance.last_update,
//             })
//         }
//         CollectionStatus::Collecting | CollectionStatus::NotStarted => {
//             Err(anyhow!("Data for this address is being collected. Please try again soon."))
//         }
//         CollectionStatus::Error(e) => {
//             Err(anyhow!("Collection failed: {}", e).into())
//         }
//     }
// }

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema(
    service: QueryService,
    collection_manager: Arc<CollectionManager>,
) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(service)
        .data(collection_manager)
        .finish()
}
