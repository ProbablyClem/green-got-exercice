use async_trait::async_trait;

use crate::models::input_transaction::InputTransaction;

#[async_trait]
pub trait TransactionHandler {
    async fn handle(&self, transaction: InputTransaction);
}