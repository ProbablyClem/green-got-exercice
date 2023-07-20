use std::sync::Arc;

use tokio::join;
mod api;
mod infra;
mod models;
mod services;

use models::config::Config;
use api::start_server;
use infra::queue::consumer::kafka_consumer;
use infra::queue::consumer::queue_consumer::QueueConsumer;

use crate::infra::queue::producer::kafka_producer::KafkaProducer;
use crate::services::output_transaction_service::handle_transaction;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let queue_config = "localhost:29092".to_string();

    let config = Config {
        queue_producer: Arc::new(KafkaProducer::new(queue_config.clone())),
    };

    let api_future = start_server(config);
    
    let consumer = kafka_consumer::KafkaConsumer::new(queue_config);
    let subscribe_future = consumer.subscribe_input_transactions(handle_transaction);

    join!(api_future, subscribe_future);
}