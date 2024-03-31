use serde::{Deserialize, Serialize};

// For DB
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all (serialize = "snake_case", deserialize = "camelCase"))]
pub struct CardDetailsDb {
    pub card_number: String,
    pub expiration_month: i32,
    pub expiration_year: i32,
    pub security_code: String,
    pub current_amount: Option<f64>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all (serialize = "camelCase", deserialize = "snake_case"))]
pub struct CardDetailsApi {
    pub card_number: String,
    pub expiration_month: i32,
    pub expiration_year: i32,
    pub security_code: String,
    pub rms_session_id: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all (serialize = "camelCase", deserialize = "camelCase"))]
pub struct TransactionQuery {
    pub page_index: i32,
    pub item_per_page: i32,
    pub end_date: String,
    pub start_date: String,
    pub preferred_language: String,
    pub rms_session_id: String
}