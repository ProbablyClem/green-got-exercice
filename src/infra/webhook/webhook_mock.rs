use async_trait::async_trait;

use crate::models::output_transaction::OutputTransaction;

use super::webhook::Webhook;

pub struct WebhookMock {
    pub list: Vec<OutputTransaction>,
}

impl WebhookMock {
    pub fn new() -> Self {
        WebhookMock {
            list: Vec::new(),
        }
    }
}

#[async_trait]
impl Webhook for WebhookMock {
    async fn send(&self, _: OutputTransaction) {
        assert!(true)
    }
}