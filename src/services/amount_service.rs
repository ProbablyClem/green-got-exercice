use crate::models::{input_transaction::InputAmount, output_transaction::OutputAmout};

impl From<InputAmount> for OutputAmout {
    fn from(input_amount: InputAmount) -> Self {
        OutputAmout {
            value: normalize_value(input_amount.value),
            currency: normalize_currency(input_amount.currency),
        }
    }
}

/// multiply by 100 and get absolute value
/// Converting euros to cents
fn normalize_value(value: f64) -> u64 {
    (value * 100.0).abs() as u64 
}

/// truncate to 3 characters and uppercase
fn normalize_currency(currency: String) -> String {
    let currency_truncated = match currency.len() > 3 {
        true => currency[0..3].to_string(),
        false => currency,
    };
    currency_truncated.to_uppercase()
}

#[cfg(test)]
mod test {
    use crate::models::input_transaction::InputAmount;
    use crate::models::output_transaction::OutputAmout;
    use crate::services::amount_service::normalize_value;

    #[test]
    fn test_normalize_value() {
        assert_eq!(normalize_value(150.0), 15000);
    }

    #[test]
    fn test_normalize_value_negative() {
        assert_eq!(normalize_value(-10.22), 1022);
    }

    #[test]
    fn test_normalize_value_zero() {
        assert_eq!(normalize_value(0.0), 0);
    }

    use crate::services::amount_service::normalize_currency;

    #[test]
    fn test_normalize_currency() {
        assert_eq!(normalize_currency("EUR".to_string()), "EUR");
    }

    #[test]
    fn test_normalize_currency_lowercase() {
        assert_eq!(normalize_currency("eur".to_string()), "EUR");
    }

    #[test]
    fn test_normalize_currency_full_name() {
        assert_eq!(normalize_currency("euros".to_string()), "EUR");
    }

    #[test]
    fn test_normalize_currency_empty() {
        assert_eq!(normalize_currency("".to_string()), "");
    }

    #[test]
    fn test_normalize_currency_short() {
        assert_eq!(normalize_currency("E".to_string()), "E");
    }

    #[test]
    fn test_from_input_amout_negative() {
        let input_amount = InputAmount {
            value: -10.22,
            currency: "euros".to_string(),
        };
        let output_amount = OutputAmout::from(input_amount);
        let expected_output_amount = OutputAmout {
            value: 1022,
            currency: "EUR".to_string(),
        };
        assert_eq!(output_amount, expected_output_amount);
    }

    #[test]
    fn test_from_input_amout_positive() {
        let input_amount = InputAmount {
            value: 150.0,
            currency: "euros".to_string(),
        };
        let output_amount = OutputAmout::from(input_amount);
        let expected_output_amount = OutputAmout {
            value: 15000,
            currency: "EUR".to_string(),
        };
        assert_eq!(output_amount, expected_output_amount);
    }
}
