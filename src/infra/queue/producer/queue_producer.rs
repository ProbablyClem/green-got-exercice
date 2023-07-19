use async_trait::async_trait;

use crate::models::input_transaction::InputTransaction;

#[async_trait]
pub trait QueueProducer {
  async fn add_input_transaction(&self, input_transaction: &InputTransaction);
}