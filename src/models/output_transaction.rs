use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct OutputTransaction {
    pub clientId: String,
    pub amount: OutputAmout,
    pub counterpart : String,
    pub rawCounterpart : Option<String>,
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
    DEBIT,
    CREDIT  ,    
}

