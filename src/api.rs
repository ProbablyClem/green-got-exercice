use std::net::SocketAddr;

use axum::{Router, routing::post};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::Config;

use crate::models::input_transaction::InputAmount;
use crate::models::input_transaction::InputTransaction;
use crate::services::input_transaction_service::receive_transaction;
use crate::services::input_transaction_service::__path_receive_transaction;

#[derive(OpenApi)]
    #[openapi(
        paths(receive_transaction),
        components(
            schemas(InputTransaction, InputAmount)
        ),
    )]
    struct ApiDoc;

/// Starts the HTTP server
pub async fn start_server(config : Config, addr : SocketAddr) {
    tracing::info!("listening on {}", addr);

    let app : Router<()> = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/transaction", post(receive_transaction))
        .with_state(config.clone());

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}