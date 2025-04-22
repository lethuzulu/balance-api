// Kafka consumer for transaction events
use crate::{common::types::TransactionEvent, data_collection::kafka::KafkaConfig};
use anyhow::{Context, Result};
use hyper::client;
use log::{error, info, debug, warn};
use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
};
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::task;

pub struct KafkaConsumer {
    /// Kafka consumer for message consumption
    consumer: StreamConsumer,
    /// Kakfa topic for transaction events
    topic: String
}

impl KafkaConsumer {
    /// Create a new TransactionConsumer
    pub fn new(config: KafkaConfig) -> Result<Self> {
        info!("Creating Kafka consumer with config: {:?}", config);

        let mut client_config = ClientConfig::new();
        client_config
            .set("bootstrap.servers", &config.brokers)
            .set("enable.auto.commit", "true")  
            .set("auto.offset.reset", "earliest")
            .set("session.timeout.ms", "6000")
            .set("max.poll.interval.ms", "300000")
            .set("group.id", "balance-api-group")
            .set("client.id", "balance-api-consumer")
            .set("heartbeat.interval.ms", "2000")
            .set("debug", "all")
            .set("enable.partition.eof", "false")
            .set("isolation.level", "read_committed")
            .set("group.instance.id", "balance-api-consumer-1")
            .set("fetch.wait.max.ms", "100")
            .set("fetch.min.bytes", "1")
            .set("fetch.max.bytes", "52428800")
            .set("max.partition.fetch.bytes", "1048576");

        // Create the Consumer
        let consumer: StreamConsumer = client_config.create().context("Failed to create consumer")?;
        info!("Successfully created Kafka consumer");

        Ok(Self {
            consumer,
            topic: config.default_topic.clone(),
        })
    }

    pub fn start(self, sender: Sender<TransactionEvent>) -> Result<()> {
        info!("Starting Kafka consumer for topic: {}", self.topic);

        let consumer = Arc::new(self.consumer);
        
        // Subscribe to the topic
        match consumer.subscribe(&[&self.topic]) {
            Ok(_) => info!("Successfully subscribed to topic: {}", self.topic),
            Err(e) => {
                error!("Failed to subscribe to topic: {}", e);
                return Err(e.into());
            }
        }

        let consumer_clone = Arc::clone(&consumer);

        task::spawn(async move {
            info!("Consumer Task Started");

            loop {
                debug!("Waiting for next message...");
                match consumer_clone.recv().await {
                    Ok(message) => {
                        debug!("Received message: {:?}", message);
                        let payload = match message.payload() {
                            Some(p) => p,
                            None => {
                                error!("Empty message payload");
                                continue;
                            }
                        };

                        debug!("Message payload: {:?}", payload);

                        let event = match serde_json::from_slice::<TransactionEvent>(payload) {
                            Ok(event) => event,
                            Err(e) => {
                                error!("Failed to deserialize message: {}", e);
                                continue;
                            }
                        };
                        
                        debug!("Successfully deserialized event: {:?}", event);
                        
                        // Send to the channel
                        if let Err(e) = sender.send(event).await {
                            error!("Failed to send transaction event: {}", e);
                        } else {
                            debug!("Successfully sent event to processor");
                        }
                    }
                    Err(KafkaError::NoMessageReceived) => {
                        debug!("No message received, continuing...");
                        continue;
                    }
                    Err(e) => {
                        error!("Error receiving message: {}", e);
                        // Add a small delay before retrying
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            }
        });

        Ok(())
    }
}
