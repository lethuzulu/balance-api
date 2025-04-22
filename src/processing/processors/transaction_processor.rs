use crate::common::types::TransactionEvent;
use anyhow::Result;
use log::info;
use tokio::sync::mpsc::Receiver;

pub struct TransactionProcessor {
    // Add fields for state here, e.g.:
    // db_connection: DatabaseConnection,
    // config: ProcessorConfig,
    // metrics: MetricsClient,
}

impl TransactionProcessor {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn start(self, mut receiver: Receiver<TransactionEvent>) {
        // Spawn a task to process received messages
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                println!("{:?}", event);
                // Here we call the actual processing logic
                if let Err(e) = self.process_transaction(event).await {
                    log::error!("Failed to process transaction: {}", e);
                }
            }
        });
    }

    pub async fn process_transaction(&self, event: TransactionEvent) -> Result<()> {
        // Move the processing logic here
        // Here you would implement the actual business logic
        //     // For example:
        //     // 1. Calculate balance changes
        //     // 2. Update the database
        //     // 3. Emit any necessary events
        //     // etc.
        //     info!("TransactionEvent {:?}", event);
        //     todo!("Implement transaction processing logic");
        //     Ok(())


        // Append the transaction to file
        let transaction_json = serde_json::to_string(&event)?;
        
        // Open file in append mode, create if it doesn't exist
        use std::fs::OpenOptions;
        use std::io::Write;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("transactions.log")?;
            
        writeln!(file, "{}", transaction_json)?;
        
        Ok(())
    }
}
