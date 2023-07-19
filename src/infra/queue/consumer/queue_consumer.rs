use async_trait::async_trait;

#[async_trait]
pub trait QueueConsumer {
    async fn subscribe_input_transactions(&self, callback: fn(&str)) -> Result<(), Box<dyn std::error::Error>>;
}
