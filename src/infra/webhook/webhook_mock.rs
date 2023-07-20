use crate::models::output_transaction::OutputTransaction;

use super::webhook::Webhook;

pub struct WebhookMock {
}

impl Webhook for WebhookMock {

    fn send(&self, output_transaction: OutputTransaction) {
        println!("WebhookMock::send({:?})", output_transaction);
    }
}