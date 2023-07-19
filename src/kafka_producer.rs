use crate::input_transaction::InputTransaction;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

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

pub async fn add_input_transaction(brokers: &str, input_transaction: &InputTransaction) {
    let topic = "input_transactions";
    let input_transaction_json =
        serde_json::to_string_pretty(&input_transaction).expect("json serialization failed");
    add_to_topic(brokers, topic, input_transaction_json).await;
}
