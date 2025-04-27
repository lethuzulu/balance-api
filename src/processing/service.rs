// Processing Service

use crate::common::types::TransactionEvent;
use crate::data_collection::kafka::KafkaConfig;
use crate::processing::kafka::consumer::KafkaConsumer;
use crate::processing::processors::transaction_processor::TransactionProcessor;
use anyhow::Result;
use tokio::sync::mpsc;

pub struct ProcessingService {
    processor: TransactionProcessor,
    consumer: KafkaConsumer,
    // You might want to add fields here for things like:
    // - Database connections
    // - Configuration
    // - Metrics
}

impl ProcessingService {
    pub fn new(kafka_config: KafkaConfig) -> Result<Self> {
        let consumer = KafkaConsumer::new(kafka_config)?;
        let processor = TransactionProcessor::new()?;
        Ok(Self {
            consumer,
            processor,
        })
    }
    pub async fn start(self) -> Result<()> {
        // info!("Starting processing service.");

        //Create channel for transaction events
        let (tx, rx) = mpsc::channel::<TransactionEvent>(1000);

        // Start the Kafka consumer & pass it the sender half of the channel
        let _ = self.consumer.start(tx);

        // Start the processor & pass it the receiver half of the channel
        let _ = self.processor.start(rx);

        Ok(())
    }
}
