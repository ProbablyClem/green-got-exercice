use std::net::SocketAddr;
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

use crate::infra::webhook::webhook_post::WebhookPost;
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

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let api_future = start_server(config, addr);

    let webhook = Box::new(WebhookPost::new(
        "https://postman-echo.com/post".to_string(),
    ));

    let output_transaction_service = Box::new(OutputTransactionService::new(webhook));
    let subscribe_future = consumer.subscribe_input_transactions(output_transaction_service);

    let _ = join!(api_future, subscribe_future);
}

//Integration tests
#[cfg(test)]
mod test {
    use std::{net::SocketAddr, sync::Arc};

    use hyper::{Body, Method, Request, StatusCode};

    use crate::{
        api::start_server,
        infra::{
            queue::{
                consumer::{mock_consumer::MockConsumer, queue_consumer::QueueConsumer},
                producer::mock_producer::MockProducer,
            },
            webhook::webhook_mock::WebhookMock,
        },
        models::config::Config,
        services::output_transaction_service::OutputTransactionService,
    };

    #[tokio::test]
    async fn test_ws_valid() {
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

    #[tokio::test]
    async fn test_ws_invalid() {
        //Missing counterpart
        let input = r#"{
            "clientId": "1234567890",
            "amount": {
                "value": 150.0,
                "currency": "euros"
            },
        }"#;

        let statut = call_ws(input).await;
        assert_eq!(statut, StatusCode::BAD_REQUEST)
    }

    //Start the serveur and call send request
    async fn call_ws(input: &str) -> StatusCode {
        tokio::spawn(async move {
            let producer = MockProducer::new();
            let config = Config {
                queue_producer: Arc::new(producer),
            };
            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            start_server(config, addr).await;
        });

        let client = hyper::Client::new();

        let response = client
            .request(
                Request::builder()
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .uri("http://localhost:3000/")
                    .body(Body::from(input.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        response.status()
    }

    //Testing that the webhook is called by the service
    #[tokio::test]
    async fn test_webhook() {
        let consumer = MockConsumer::new();

        let webhook = Box::new(WebhookMock::new());

        let output_transaction_service = Box::new(OutputTransactionService::new(webhook));
        consumer
            .subscribe_input_transactions(output_transaction_service)
            .await
            .unwrap();
    }
}
