use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use rdkafka::config;
use tokio::join;

use std::net::SocketAddr;

mod input_transaction;
mod kafka_consumer;
mod kafka_producer;

use input_transaction::InputTransaction;
use kafka_consumer::subscribe_input_transactions;
use kafka_producer::add_input_transaction;

#[derive(Clone)]
pub struct Config {
    queue_config: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    let config = Config {
        queue_config: "localhost:29092".to_string(),
    };

    let app : Router<()> = Router::new()
        .route("/", post(receive_transaction))
        .with_state(config.clone());

    let axum_future = axum::Server::bind(&addr).serve(app.into_make_service());

    let consumer_future = subscribe_input_transactions(&config.queue_config);
    join!(axum_future, consumer_future);
}

async fn receive_transaction(
    State(config): State<Config>,
    Json(payload): Json<InputTransaction>,
) -> impl IntoResponse {
    tracing::info!("received payload: {:?}", payload);

    let brokers = &config.queue_config;
    add_input_transaction(brokers, &payload).await;
    StatusCode::OK
}
