use crate::{models::input_transaction::InputTransaction, Config};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

pub async fn receive_transaction(
    State(config): State<Config>,
    Json(payload): Json<InputTransaction>,
) -> impl IntoResponse {
    tracing::info!("received payload: {:?}", payload);

    config.queue_producer.add_input_transaction(&payload).await;
    StatusCode::OK
}
