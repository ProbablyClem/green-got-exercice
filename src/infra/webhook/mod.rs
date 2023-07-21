pub mod webhook_mock;
pub mod webhook_post;

use async_trait::async_trait;

use crate::models::output_transaction::OutputTransaction;

#[async_trait]
pub trait Webhook {
    async fn send(&self, output_transaction: OutputTransaction);
}