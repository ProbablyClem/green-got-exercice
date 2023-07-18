use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InputTransaction {
    clientId: String,
    amount: Amount,
    counterpart : String
}

#[derive(Deserialize, Debug)]
pub struct Amount {
    value: f64,
    currency: String,
}