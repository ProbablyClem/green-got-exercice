use async_trait::async_trait;

use crate::{services::output_transaction_service::OutputTransactionService, models::input_transaction::{InputTransaction, InputAmount}};

use super::{queue_consumer::QueueConsumer, kafka_consumer::KafkaConsumer};

pub struct MockConsumer {}

impl MockConsumer {
    pub fn new() -> Self {
        MockConsumer {}
    }
}

#[async_trait]
impl QueueConsumer for MockConsumer {
    async fn subscribe_input_transactions(&self, service: OutputTransactionService) -> Result<(), Box<dyn std::error::Error>> {
       for input_transaction in getMockList() {
            let input_string = serde_json::to_string(&input_transaction)?;
            service.receive(input_string);
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