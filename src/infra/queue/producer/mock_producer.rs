use async_trait::async_trait;

use crate::models::input_transaction::InputTransaction;

use super::queue_producer::QueueProducer;

pub struct MockProducer {}

impl MockProducer {
    pub fn new() -> Self {
        MockProducer {}
    }
}

#[async_trait]
impl QueueProducer for MockProducer {
    async fn add_input_transaction(&self, _input_transaction: &InputTransaction) {
        println!("MockProducer::add_input_transaction");
    }
}