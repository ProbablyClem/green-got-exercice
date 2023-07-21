use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputTransaction {
    pub clientId: String,
    pub amount: InputAmount,
    pub counterpart : String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InputAmount {
    pub value: f64,
    pub currency: String,
}

impl From<String> for InputTransaction {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}