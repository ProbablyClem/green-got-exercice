use async_trait::async_trait;

use crate::{services::{output_transaction_service::OutputTransactionService, transaction_handler::TransactionHandler}, models::input_transaction::{InputTransaction, InputAmount}};

use super::{queue_consumer::QueueConsumer, kafka_consumer::KafkaConsumer};

pub struct MockConsumer {}

impl MockConsumer {
    pub fn new() -> Self {
        MockConsumer {}
    }
}

#[async_trait]
impl QueueConsumer for MockConsumer {
    async fn subscribe_input_transactions(&self, service: Box<dyn TransactionHandler + Send + Sync>) -> Result<(), Box<dyn std::error::Error>> {
       for input_transaction in getMockList() {
            service.handle(input_transaction).await;
        }
        Ok(())
    }
}

fn getMockList() -> Vec<InputTransaction>{
    let mut mock_list = vec![];
    mock_list.push(InputTransaction {
        clientId: "1234567890".to_string(),
        amount: InputAmount {
            value: -10.22,
            currency: "euros".to_string(),
        },
        counterpart: "SCNF VA122345 dt: 01/01/2020".to_string(),
    });
    mock_list.push(InputTransaction {
        clientId: "1234567890".to_string(),
        amount: InputAmount {
            value: 150.0,
            currency: "euros".to_string(),
        },
        counterpart: "papa".to_string(),
    });
    mock_list
}