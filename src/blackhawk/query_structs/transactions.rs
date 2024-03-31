use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    balances: Balances,
    ending_item_index: i32,
    number_of_items_in_page: i32,
    page_index: i32,
    starting_item_index: i32,
    total_number_of_items: i32,
    total_number_of_pages: i32,
    pub transactions: Vec<TransactionL2> 
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Balances {
    closing_balance: f64,
    currency_code: String,
    opening_balance: f64,
    pending_balance: f64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionL2 {
    pub amount: f64,
    currency: String,
    pub description: String,
    detail: Detail,
    pub id: String,
    pub merchant_description: String,
    running_balance: f64,
    pub transaction_date: String,
    pub transaction_type: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Detail {
    merchant_address: String,
    merchant_name: String,
    settlement_amount: f32,
    settlement_currency: String,
    transaction_desctription: String
}

