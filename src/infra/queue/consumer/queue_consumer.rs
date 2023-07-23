

use async_trait::async_trait;

use crate::services::transaction_handler::TransactionHandler;

#[async_trait]
pub trait QueueConsumer<'a, T : TransactionHandler+ Send + Sync>  {
    async fn subscribe_input_transactions(
        &self,
        service: &'a T
    ) -> Result<(), Box<dyn std::error::Error>>;
}