// Kafka consumer for transaction events
use crate::common::types::TransactionEvent;
use anyhow::{Context, Result};
use log::{error, info};
use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::task;

pub struct TransactionConsumer {
    consumer: StreamConsumer,
    topic: String,
}

impl TransactionConsumer {
    /// Create a new TransactionConsumer
    pub fn new(topic: &str) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            // .set("group.id", &config.consumer_group)
            // .set("bootstrap.servers", &config.bootstrap_servers)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .set("session.timeout.ms", "6000")
            .set("max.poll.interval.ms", "300000")
            .create()
            .context("Failed to create consumer")?;

        Ok(Self {
            consumer,
            topic: topic.to_string(),
        })
    }

    pub fn start(self, sender: Sender<TransactionEvent>) -> Result<()> {
        info!("Starting Kafka consumer for topic: {}", self.topic);

        let consumer = Arc::new(self.consumer);
        consumer.subscribe(&[&self.topic])?;

        let consumer_clone = Arc::clone(&consumer);

        task::spawn(async move {
            info!("Consumer Task started");

            loop {
                match consumer_clone.recv().await {
                    Ok(message) => {
                        let payload = match message.payload() {
                            Some(p) => p,
                            None => {
                                error!("Empty message payload");
                                continue;
                            }
                        };

                        let event = match serde_json::from_slice::<TransactionEvent>(payload) {
                            Ok(event) => event,
                            Err(e) => {
                                error!("Failed to deserialize message: {}", e);
                                continue;
                            }
                        };
                        // Send to the channel
                        if let Err(e) = sender.send(event).await {
                            error!("Failed to send transaction event: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Error receiving message: {}", e);
                    }
                }
            }
        });

        Ok(())
    }
}
