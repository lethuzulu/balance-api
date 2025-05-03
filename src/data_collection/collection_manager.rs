use crate::data_collection::service::DataCollectionService;
use anyhow::{Context, Result};
use hypersync_client::format::FixedSizeData;
use redis::Commands;
use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum CollectionStatus {
    Collecting,
    Ready,
    NotStarted,
    Error(String),
}

impl fmt::Display for CollectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionStatus::Collecting => write!(f, "collecting"),
            CollectionStatus::Ready => write!(f, "ready"),
            CollectionStatus::NotStarted => write!(f, "not_started"),
            CollectionStatus::Error(e) => write!(f, "error:{}", e),
        }
    }
}

impl FromStr for CollectionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "collecting" => Ok(CollectionStatus::Collecting),
            "ready" => Ok(CollectionStatus::Ready),
            "not_started" => Ok(CollectionStatus::NotStarted),
            s if s.starts_with("error:") => {
                Ok(CollectionStatus::Error(s["error:".len()..].to_string()))
            }
            _ => Err(()),
        }
    }
}

pub struct CollectionManager {
    pub redis_client: redis::Client,
    pub data_collector: Arc<DataCollectionService>,
}

impl CollectionManager {
    pub fn new(data_collector: Arc<DataCollectionService>, redis_client: redis::Client) -> Self {
        Self {
            redis_client,
            data_collector,
        }
    }

    // Use Redis to cache and check collection status
    pub async fn get_or_trigger_collection(
        &self,
        address: String,
        block_number: u64,
    ) -> Result<CollectionStatus> {
        let mut redis_conn = self
            .redis_client
            .get_connection()
            .context("Failed to connect to redis")?;
        let key = format!("address_status:{}", address);
        let status = redis_conn.get::<_, Option<String>>(&key).context(format!(
            "Failed to get status for address {} from Redis",
            address
        ))?;

        match status {
            Some(status_str) => match CollectionStatus::from_str(&status_str) {
                Ok(status) => Ok(status),
                Err(_) => Ok(CollectionStatus::Error(
                    "Invalid status in Redis".to_string(),
                )),
            },
            None => {
                // Set as collecting and trigger collection
                let _: () = redis_conn
                    .set(&key, CollectionStatus::Collecting.to_string())
                    .unwrap_or(());

                // spawn a task for collecting
                let data_collector = self.data_collector.clone();
                tokio::spawn(async move {
                    let address = FixedSizeData::<20>::try_from(
                        hex::decode(address.trim_start_matches("0x"))
                            .unwrap()
                            .as_slice(),
                    )
                    .unwrap();

                    let deployment_block = data_collector
                        .client
                        .get_deployment_block(address.clone())
                        .await;

                    match data_collector
                        .collect_historical_transactions(
                            address,
                            deployment_block,
                            Some(block_number),
                        )
                        .await
                    {
                        Ok(_) => redis_conn
                            .set(&key, CollectionStatus::Ready.to_string())
                            .unwrap_or(()),
                        Err(e) => redis_conn
                            .set(&key, CollectionStatus::Error(e.to_string()).to_string())
                            .unwrap_or(()),
                    }
                });
                Ok(CollectionStatus::Collecting)
            }
        }
    }

    async fn collect_for_address(&self, address: String) -> Result<()> {
        // We can customie from_block/to_block as needed
        let address_bytes = hex::decode(address.trim_start_matches("0x"))?;
        let fixed_address = FixedSizeData::<20>::try_from(address_bytes.as_slice())?;
        self.data_collector
            .collect_historical_transactions(fixed_address, 0, None)
            .await?;
        Ok(())
    }
}
