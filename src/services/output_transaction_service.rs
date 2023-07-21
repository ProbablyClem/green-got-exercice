use async_trait::async_trait;

use crate::{
    infra::webhook::webhook::Webhook,
    models::{
        input_transaction::InputTransaction,
        output_transaction::{Direction, OutputAmout, OutputTransaction},
    },
};

use super::{logo_service::{LogoService, LogoServiceMap}, transaction_handler::TransactionHandler};

pub struct OutputTransactionService {
    webhook: Box<dyn Webhook + Send + Sync>,
}


impl OutputTransactionService {
    pub fn new(webhook: Box<dyn Webhook + Send + Sync>) -> Self {
        OutputTransactionService { webhook } 
    }
}

#[async_trait]
impl TransactionHandler for OutputTransactionService {
    async fn handle(&self, input_transaction: InputTransaction) {
        let input_transaction = InputTransaction::from(input_transaction);
        let output_transaction = OutputTransaction::from(input_transaction);
        self.webhook.send(output_transaction).await
    }
}

impl From<InputTransaction> for OutputTransaction {
    fn from(input_transaction: InputTransaction) -> Self {
        let direction = get_direction(input_transaction.amount.value);
        let counterpart = get_first_word(&input_transaction.counterpart);
        let rawCounterpart = match counterpart == input_transaction.counterpart {
            true => None,
            false => Some(input_transaction.counterpart),
        };

        let logo_service = LogoServiceMap::new();
        let logo = logo_service.get_logo(&counterpart);

        OutputTransaction {
            clientId: input_transaction.clientId,
            amount: OutputAmout::from(input_transaction.amount),
            counterpart,
            rawCounterpart,
            logo,
            direction,
        }
    }
}

fn get_direction(value: f64) -> Direction {
    match value > 0.0 {
        true => Direction::CREDIT,
        false => Direction::DEBIT,
    }
}

fn get_first_word(string: &str) -> String {
    string
        .split_ascii_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

#[cfg(test)]
mod test {
    use crate::models::input_transaction::InputTransaction;
    use crate::models::output_transaction::OutputTransaction;
    use crate::{
        models::{
            input_transaction::InputAmount,
            output_transaction::{self, Direction, OutputAmout},
        },
        services::output_transaction_service::get_direction,
    };

    #[test]
    fn test_get_direction_debit() {
        assert_eq!(get_direction(1.0), Direction::CREDIT);
    }

    #[test]
    fn test_get_direction_credit() {
        assert_eq!(get_direction(-1.0), Direction::DEBIT);
    }

    #[test]
    fn test_get_direction_zero() {
        assert_eq!(get_direction(0.0), Direction::DEBIT);
    }

    #[test]
    fn test_get_first_word() {
        assert_eq!(super::get_first_word("Hello World"), "Hello");
    }

    #[test]
    fn test_get_first_word_empty() {
        assert_eq!(super::get_first_word(""), "");
    }

    #[test]
    fn test_from_input_transaction_complex_counterpart() {
        let input_transaction = InputTransaction {
            clientId: "1234567890".to_string(),
            amount: InputAmount {
                value: -10.22,
                currency: "euros".to_string(),
            },
            counterpart: "SCNF VA122345 dt: 01/01/2020".to_string(),
        };
        let output_transaction = OutputTransaction {
            clientId: "1234567890".to_string(),
            amount: OutputAmout {
                value: 1022,
                currency: "EUR".to_string(),
            },
            counterpart: "SCNF".to_string(),
            rawCounterpart: Some("SCNF VA122345 dt: 01/01/2020".to_string()),
            logo: Some("/companies/logo-sncf.svg".to_string()),
            direction: Direction::DEBIT,
        };
        assert_eq!(
            OutputTransaction::from(input_transaction),
            output_transaction
        );
    }

    #[test]
    fn test_from_input_transaction_simple_counterpart() {
        let input_transaction = InputTransaction {
            clientId: "1234567890".to_string(),
            amount: InputAmount {
                value: 150.0,
                currency: "euros".to_string(),
            },
            counterpart: "papa".to_string(),
        };
        let output_transaction = OutputTransaction {
            clientId: "1234567890".to_string(),
            amount: OutputAmout {
                value: 15000,
                currency: "EUR".to_string(),
            },
            counterpart: "papa".to_string(),
            rawCounterpart: None,
            logo: None,
            direction: Direction::CREDIT,
        };
        assert_eq!(
            OutputTransaction::from(input_transaction),
            output_transaction
        );
    }
}
