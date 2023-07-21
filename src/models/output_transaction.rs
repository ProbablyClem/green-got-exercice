use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct OutputTransaction {
    #[serde(rename = "clientId")]
    pub client_id: String,
    pub amount: OutputAmout,
    pub counterpart : String,
    #[serde(rename = "rawCounterpart")]
    pub rawcounterpart : Option<String>,
    pub logo : Option<String>,
    pub direction: Direction,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct OutputAmout {
    pub value: u64,
    pub currency: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum Direction {
    #[serde(rename = "DEBIT")]
    Debit,
    #[serde(rename = "CREDIT")]
    Credit,    
}

