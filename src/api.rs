use std::net::SocketAddr;

use axum::{Router, routing::post};

use crate::Config;

use crate::services::input_transaction_service::receive_transaction;

pub async fn start_server(config : Config) {
   let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    let app : Router<()> = Router::new()
        .route("/", post(receive_transaction))
        .with_state(config.clone());

    axum::Server::bind(&addr).serve(app.into_make_service());
}