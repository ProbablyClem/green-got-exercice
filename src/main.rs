use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use std::net::SocketAddr;

mod input_transaction;
mod kafka_producer;

use input_transaction::InputTransaction;
use kafka_producer::add_input_transaction;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", post(receive_transaction));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn receive_transaction(Json(payload): Json<InputTransaction>) -> impl IntoResponse {
    tracing::info!("received payload: {:?}", payload);

    let brokers = "localhost:29092";
    add_input_transaction(brokers, &payload).await;
    StatusCode::OK
}
