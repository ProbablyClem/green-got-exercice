use async_trait::async_trait;
use hyper::{Request, Method, Body};
use tracing::info;

use crate::models::output_transaction::OutputTransaction;

use super::Webhook;

pub struct WebhookPost {
    url: String,
}

impl WebhookPost {
    pub fn new(url: String) -> Self {
        WebhookPost { url }
    }
}

#[async_trait]
impl Webhook for WebhookPost {
    async fn send(&self, output_transaction: OutputTransaction) {
        let client = hyper::Client::new();
        let transaction_string = serde_json::to_string(&output_transaction).unwrap();
        let response = client
            .request(
                Request::builder()
                    .method(Method::POST)
                    .header("content-type", "application/json")
                    .uri(&self.url)
                    .body(Body::from(transaction_string))
                    .unwrap(),
            )
            .await
            .unwrap();
        info!("response status : {}", response.status());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::output_transaction::OutputTransaction;
    use crate::infra::webhook::Webhook;
    use crate::models::output_transaction::Direction;
    use crate::models::output_transaction::OutputAmout;

    #[tokio::test]
    async fn test_webhook_post() {
        let webhook = WebhookPost::new("http://postman-echo.com/post/".to_string());
        let output_transaction = OutputTransaction {
            client_id: "1234567890".to_string(),
            amount: OutputAmout {
                value: 150,
                currency: "euros".to_string(),
            },
            counterpart: "papa".to_string(),
            rawcounterpart: None,
            logo: None,
            direction: Direction::Credit,
        };
        webhook.send(output_transaction).await;
    }
}