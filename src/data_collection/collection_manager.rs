
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use anyhow::{Result, Context, anyhow};
use crate::data_collection::service::DataCollectionService;
use hypersync_client::format::FixedSizeData;


#[derive(Debug, Clone)]
pub enum CollectionStatus {
    NotStarted,
    Collecting,
    Ready,
    Error(String)
}

#[derive(Debug, Clone)]
pub struct AddressMeta {
    pub status: CollectionStatus
}

pub struct CollectionManager {
    pub address_map: Arc<Mutex<HashMap<String, AddressMeta>>>,
    pub data_collector: Arc<DataCollectionService>
}

impl CollectionManager {
    pub fn new(data_collector: Arc<DataCollectionService>) -> Self {
        Self {address_map: Arc::new(Mutex::new(HashMap::new())),
            data_collector
        }
    }

    //TODO: review implementation
    pub async fn get_or_trigger_collection(&self, address: String) -> CollectionStatus {
        let mut map = self.address_map.lock().unwrap();
        match map.get(&address) {
            Some(meta) => meta.status.clone(),
            None => {
                // Insert as collecting and trigger collection
                map.insert(address.clone(), AddressMeta { status: CollectionStatus::Collecting });
                let data_collector = Arc::clone(&self.data_collector);
                let address_clone = address.clone();
                let address_map = Arc::clone(&self.address_map);
                tokio::spawn(async move {
                    let result = data_collector.collect_historical_transactions(
                        FixedSizeData::<20>::try_from(hex::decode(address_clone.trim_start_matches("0x")).unwrap().as_slice()).unwrap(),
                        0,
                        None
                    ).await;
                    let mut map = address_map.lock().unwrap();
                    match result {
                        Ok(_) => {
                            map.insert(address_clone, AddressMeta { status: CollectionStatus::Ready });
                        },
                        Err(e) => {
                            map.insert(address_clone, AddressMeta { status: CollectionStatus::Error(e.to_string()) });
                        },
                    };
                });
                CollectionStatus::Collecting
            }
        }
    }

    async fn collect_for_address(&self, address: String) -> Result<()>{
        // We can customie from_block/to_block as needed
        let address_bytes = hex::decode(address.trim_start_matches("0x"))?;
        let fixed_address = FixedSizeData::<20>::try_from(address_bytes.as_slice())?;
        self.data_collector.collect_historical_transactions(fixed_address, 0, None).await?;
        Ok(())
    }
} 