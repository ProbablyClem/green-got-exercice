use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InputTransaction {
    clientId: String,
    amount: Amount,
    counterpart : String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Amount {
    value: f64,
    currency: String,
}