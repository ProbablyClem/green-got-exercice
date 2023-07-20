use std::sync::Arc;

use async_trait::async_trait;

use crate::services::output_transaction_service::OutputTransactionService;

#[async_trait]
pub trait QueueConsumer {
    async fn subscribe_input_transactions(&self, service : OutputTransactionService) -> Result<(), Box<dyn std::error::Error>>;
}
