use std::time::Duration;

use axum::Error;
use rdkafka::config::ClientConfig;
use rdkafka::Message;

use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaResult;
use tracing::warn;

use crate::input_transaction::InputTransaction;

fn create_consumer(brokers: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", brokers)
         .set("group.id", "service")
        .set("enable.auto.commit", "false")
        .create()
        .unwrap()
}

pub async fn subscribe_input_transactions(brokers: &str) -> Result<(), Box<dyn std::error::Error>>{
    let consumer = create_consumer(brokers);
    let topic = "input_transactions";
    consumer
        .subscribe(&[topic]).expect("Can't subscribe to specified topic");
    loop {
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                match m.payload_view::<str>() {
                    None => (),
                    Some(Ok(s)) => print(s),
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
    Ok(())
}

fn print(s: &str) {
    let input_transaction =
        serde_json::from_str::<InputTransaction>(s).expect("json deserialization failed");
    println!("Input transaction: {:#?}", input_transaction);
}
