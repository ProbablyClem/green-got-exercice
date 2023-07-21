use std::sync::Arc;

use tokio::join;
mod api;
mod infra;
mod models;
mod services;

use api::start_server;
use infra::queue::consumer::queue_consumer::QueueConsumer;
use models::config::Config;

use crate::infra::queue::consumer::kafka_consumer::KafkaConsumer;
use crate::infra::queue::producer::kafka_producer::KafkaProducer;
use crate::infra::webhook::webhook_mock::WebhookMock;
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

    let webhook = Box::new(WebhookMock::new());

    let output_transaction_service = OutputTransactionService::new(webhook);
    let subscribe_future = consumer.subscribe_input_transactions(output_transaction_service);

    join!(api_future, subscribe_future);
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use hyper::{Body, Method, Request, StatusCode};

    use crate::{
        api::start_server,
        infra::queue::producer::mock_producer::MockProducer,
        models::{
            config::Config,
            input_transaction::{InputAmount, InputTransaction},
        },
    };

    #[tokio::test]
    async fn test_ws_valide() {
        let input = r#"{
            "clientId": "1234567890",
            "amount": {
                "value": 150.0,
                "currency": "euros"
            },
            "counterpart": "papa"
        }"#;
        let statut = call_ws(input).await;
        assert_eq!(statut, StatusCode::OK)
    }

    async fn call_ws(input: &str) -> StatusCode {
        tokio::spawn(async move {
            let producer = MockProducer::new();
            let config = Config {
                queue_producer: Arc::new(producer),
            };

            start_server(config).await;
        });

        let client = hyper::Client::new();



        let response = client
            .request(
                Request::builder()
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .uri("http://127.0.0.1:3000/")
                    .body(Body::from(input.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        response.status()
    }
}
