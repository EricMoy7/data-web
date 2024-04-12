use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardDetailsRequest {
    pub card_number: String,
    pub expiration_month: i32,
    pub expiration_year: i32,
    pub security_code: String,
}

impl From<CardDetailsDb> for CardDetailsRequest {
    fn from(card: CardDetailsDb) -> Self {
        Self {
            card_number: card.card_number,
            expiration_month: card.expiration_month,
            expiration_year: card.expiration_year,
            security_code: card.security_code,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardDetailsDb {
    pub card_number: String,
    pub expiration_month: i32,
    pub expiration_year: i32,
    pub security_code: String,
    pub current_amount: Option<f64>
}
