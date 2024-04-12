use serde::{Deserialize, Serialize};
use async_graphql::{Object, Context};

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

#[Object]
impl CardDetailsDb {
    // define methods for each field you want to expose in your GraphQL API
    // for example, if CardDetailsDb has a field named `card_id`, you might do:
    async fn card_number(&self, ctx: &Context<'_>) -> String {
        self.card_number.chars().rev().take(4).collect::<String>().chars().rev().collect()
    }

    async fn current_amount(&self, ctx: &Context<'_>) -> f64 {
        self.current_amount.expect("This is not a number")
    }

    // repeat for other fields...
}