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
use tracing::info;

use crate::infra::queue::consumer::kafka_consumer::KafkaConsumer;
use crate::infra::queue::producer::kafka_producer::KafkaProducer;

use crate::infra::webhook::webhook_post::WebhookPost;
use crate::services::output_transaction_service::OutputTransactionService;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variables from the .env file
    tracing_subscriber::fmt::init();

    let queue_config = std::env::var("KAFKA_HOST").expect("KAFKA_HOST env variable must be set.");
    info!("Kafka host : {}", queue_config);
    
    let producer = KafkaProducer::new(queue_config.clone());
    let consumer = KafkaConsumer::new(queue_config);
    let config = Config {
        queue_producer: Arc::new(producer),
    };

    let port = std::env::var("PORT").expect("PORT env variable must be set.").parse::<u16>().expect("PORT env variable must be a number.");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let api_future = start_server(config, addr);

    let webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL env variable must be set.");
    let webhook = Box::new(WebhookPost::new(webhook_url));

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
        infra::queue::producer::mock_producer::MockProducer,
        models::config::Config,
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
}
