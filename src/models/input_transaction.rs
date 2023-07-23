use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct InputTransaction {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub amount: InputAmount,
    pub counterpart : String
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub struct InputAmount {
    pub value: f64,
    pub currency: String,
}

impl From<String> for InputTransaction {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}