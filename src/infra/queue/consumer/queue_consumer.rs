use std::sync::Arc;

use async_trait::async_trait;

use crate::services::transaction_handler::TransactionHandler;

#[async_trait]
pub trait QueueConsumer {
    async fn subscribe_input_transactions(
        &self,
        service: Box<dyn TransactionHandler + Send + Sync>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}