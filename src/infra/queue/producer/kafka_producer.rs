use crate::models::input_transaction::InputTransaction;
use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;
use crate::infra::queue::producer::queue_producer::QueueProducer;
pub struct KafkaProducer {
    brokers: String,
}

impl KafkaProducer {
    pub fn new(brokers: String) -> KafkaProducer {
        KafkaProducer { brokers }
    }
}

#[async_trait]
impl QueueProducer for KafkaProducer {
    async fn add_input_transaction(&self, input_transaction: &InputTransaction) {
        let brokers = &self.brokers;
        let input_transaction_json =
            serde_json::to_string_pretty(&input_transaction).expect("json serialization failed");
        add_to_topic(brokers, "input_transactions", input_transaction_json).await;
    }
}

fn create_producer(brokers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .unwrap()
}

async fn add_to_topic(brokers: &str, topic: &str, message: String) {
    let producer = create_producer(brokers);

    let record = FutureRecord::<String,String>::to(topic).payload(&message);

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(_) => println!("Written"),
        Err(e) => eprintln!("Error writting message: {:?}", e),
    }
}
