
use async_trait::async_trait;
use rdkafka::{Message, ClientConfig};

use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::error::KafkaResult;
use tracing::warn;

use crate::infra::queue::consumer::queue_consumer::QueueConsumer;
pub struct KafkaConsumer {
    brokers: String,
}

impl KafkaConsumer {
    pub fn new(brokers: String) -> KafkaConsumer {
        KafkaConsumer { brokers }
    }
}

#[async_trait]
impl QueueConsumer for KafkaConsumer {
    async fn subscribe_input_transactions(&self, callback: fn(&str)) -> Result<(), Box<dyn std::error::Error>> {
        let consumer = create_consumer(&self.brokers);
        let topic = "input_transactions";
        consumer
            .subscribe(&[topic])
            .expect("Can't subscribe to specified topic");
        loop {
            match consumer.recv().await {
                Err(e) => warn!("Kafka error: {}", e),
                Ok(m) => {
                    match m.payload_view::<str>() {
                        None => (),
                        Some(Ok(s)) => callback(s),
                        Some(Err(e)) => {
                            warn!("Error while deserializing message payload: {:?}", e);
                        }
                    };

                    consumer
                        .commit_message(&m, rdkafka::consumer::CommitMode::Async)
                        .unwrap();
                }
            }
        }
    }
}


fn create_consumer(brokers: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
         .set("group.id", "service")
        .set("enable.auto.commit", "false")
        .create()
        .unwrap()
}