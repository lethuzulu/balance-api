// Processing Service

use crate::common::types::TransactionEvent;
use crate::processing::kafka::consumer::TransactionConsumer;
use crate::processing::processors::transaction_processor::TransactionProcessor;
use anyhow::Result;
use log::info;
use tokio::sync::mpsc;

pub struct ProcessingService {
    consumer: TransactionConsumer,
    processor: TransactionProcessor,
    // You might want to add fields here for things like:
    // - Database connections
    // - Configuration
    // - Metrics
}

impl ProcessingService {
    pub fn new() -> Result<Self> {
        let consumer = TransactionConsumer::new("topic")?;
        let processor = TransactionProcessor::new()?;
        Ok(Self {
            consumer,
            processor,
        })
    }
    pub async fn start(self) -> Result<()> {
        info!("Starting processing service.");

        //Create channel for transaction events
        let (tx, rx) = mpsc::channel::<TransactionEvent>(1000);

        // Start the Kafka consumer & pass it the sender half of the channel
        let _ = self.consumer.start(tx);

        // Start the processor & pass it the receiver half of the channel
        let _ = self.processor.start(rx);

        Ok(())
    }
}
