use std::{cell::RefCell, sync::RwLock};

use async_trait::async_trait;

use crate::models::output_transaction::OutputTransaction;

use super::Webhook;

pub struct WebhookMock {
    pub sent_transactions: RwLock<Vec<OutputTransaction>>,
}

impl WebhookMock {
    #[allow(dead_code)]
    pub fn new() -> Self {
        WebhookMock {
            sent_transactions: RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl Webhook for WebhookMock {
    async fn send(&self, output_transaction: OutputTransaction) {
        self.sent_transactions.write().unwrap().push(output_transaction);
    }
}
