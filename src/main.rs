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

use crate::infra::queue::consumer::kafka_consumer::KafkaConsumer;
use crate::infra::queue::producer::kafka_producer::KafkaProducer;
use crate::infra::webhook::webhook_mock::WebhookMock;
use crate::services::logo_service::LogoServiceMap;
use crate::services::output_transaction_service::OutputTransactionService;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let queue_config = "localhost:29092".to_string();
    let producer = KafkaProducer::new(queue_config.clone());
    let consumer = KafkaConsumer::new(queue_config);
    let config = Config {
        queue_producer: Arc::new(producer),
    };

    let api_future = start_server(config);


    let webhook = Box::new(WebhookMock {});

    let output_transaction_service = OutputTransactionService::new(webhook);
    let subscribe_future = consumer.subscribe_input_transactions(output_transaction_service);

    join!(api_future, subscribe_future);
}