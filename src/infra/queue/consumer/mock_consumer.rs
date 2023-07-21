use async_trait::async_trait;

use crate::{
    models::input_transaction::{InputAmount, InputTransaction},
    services::transaction_handler::TransactionHandler,
};

use super::queue_consumer::QueueConsumer;

pub struct MockConsumer {}

impl MockConsumer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MockConsumer {}
    }
}

#[async_trait]
impl QueueConsumer for MockConsumer {
    async fn subscribe_input_transactions(
        &self,
        service: Box<dyn TransactionHandler + Send + Sync>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for input_transaction in get_mock_list() {
            service.handle(input_transaction).await;
        }
        Ok(())
    }
}

fn get_mock_list() -> Vec<InputTransaction> {
    vec![
        InputTransaction {
            client_id: "1234567890".to_string(),
            amount: InputAmount {
                value: -10.22,
                currency: "euros".to_string(),
            },
            counterpart: "SCNF VA122345 dt: 01/01/2020".to_string(),
        },
        InputTransaction {
            client_id: "1234567890".to_string(),
            amount: InputAmount {
                value: 150.0,
                currency: "euros".to_string(),
            },
            counterpart: "papa".to_string(),
        },
    ]
}
