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

impl Webhook for WebhookMock {

    fn send(&self, output_transaction: OutputTransaction) {
        assert!(true)
    }
}