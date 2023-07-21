
use async_trait::async_trait;
use rdkafka::{Message, ClientConfig};

use rdkafka::consumer::{StreamConsumer, Consumer};
use tracing::warn;

use crate::infra::queue::consumer::queue_consumer::QueueConsumer;
use crate::models::input_transaction;
use crate::services::output_transaction_service::OutputTransactionService;
use crate::services::transaction_handler::TransactionHandler;
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
    async fn subscribe_input_transactions(&self, service: Box<dyn TransactionHandler + Send + Sync>) -> Result<(), Box<dyn std::error::Error>> {
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
                        Some(Ok(s)) => {
                            let input_transaction = input_transaction::InputTransaction::from(s.to_string());
                            let _ = service.handle(input_transaction).await;
                        }
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