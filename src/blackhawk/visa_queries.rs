use reqwest::{self, cookie::{CookieStore, Jar}, Url};
use std::{error::Error, sync::Arc, fs};
use serde_json::Value;

use super::structures::{CardDetailsApi, CardDetailsDb, TransactionQuery};

// We should consider paramter as & if possible
pub async fn get_balance(card_details: CardDetailsDb) -> Result<serde_json::Value, Box<dyn Error>> {
    let rms_session_id = fs::read_to_string("rms.txt").expect("Unable to read file");

    // Modify struct fields for this 
    let card_details_subset = CardDetailsApi {
        card_number: card_details.card_number,
        expiration_month: card_details.expiration_month,
        expiration_year: card_details.expiration_year,
        security_code: card_details.security_code,
        rms_session_id
    };

    let base_url: Url = Url::parse("https://mygift.giftcardmall.com").unwrap();
    let endpoint = "/api/card/getCardBalanceSummary".to_string();

    let jar = Arc::new(Jar::default());

    let cookies = fs::read_to_string("cookies.txt").expect("Unable to read file");
    jar.add_cookie_str(&cookies, &base_url);

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.clone())
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Host", "www.mcgift.giftcardmall.com".parse()?);
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0".parse()?);
    headers.insert("Accept", "application/json, text/plain, */*".parse()?);
    headers.insert("Accept-Language", "en-US,en;q=0.5".parse()?);
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("Sec-Fetch-Dest", "empty".parse()?);
    headers.insert("Sec-Fetch-Mode", "cors".parse()?);
    headers.insert("Sec-Fetch-Site", "same-origin".parse()?);
    headers.insert("TE", "trailers".parse()?);
    // headers.insert("Cookie", "datadome=ERxD5vxe8AOoLNByfQBXJRz9igb4eG1sLX~BuLRv~ZNV1Ao3VB0MLQjhCAUuxThHX~AieI90toqGTBNGVGicY2sdg~6QVtcNwGX22_lQTA6fz7lbWgIx~OfcAos8BSGb".parse()?);

    // let json_string = serde_json::to_string(&self).unwrap();
    let json: serde_json::Value  = serde_json::to_value(&card_details_subset).unwrap();
    // println!("{:#?}", json);
    
    let request = client.post(base_url.clone().join(&endpoint).unwrap())
        .headers(headers)
        .json(&json)
        .send()
        .await?
        .text()
        .await?;

    let cookies = jar.cookies(&base_url).unwrap();
    fs::write("cookies.txt", cookies.to_str().unwrap()).expect("Unable to write file");

    let json_object: Value = serde_json::from_str(&request.as_str())?;

    println!("{:#?}", json_object);
    // println!("{:#?}", &json_object["success"]);

    Ok(json_object)
}

pub async fn get_transactions(transaction_query: &TransactionQuery, token: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let base_url: Url = Url::parse("https://mygift.giftcardmall.com").unwrap();
    let endpoint = "/api/card/getCardTransactions".to_string();

    let jar = Arc::new(Jar::default());

    let cookies = fs::read_to_string("cookies.txt").expect("Unable to read file");
    jar.add_cookie_str(&cookies, &base_url);

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(jar.clone())
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Host", "mygift.giftcardmall.com".parse()?);
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0".parse()?);
    headers.insert("Accept", "application/json, text/plain, */*".parse()?);
    headers.insert("Accept-Language", "en-US,en;q=0.5".parse()?);
    // headers.insert("Accept-Encoding", "gzip, deflate, br".parse()?);
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("Sec-Fetch-Dest", "empty".parse()?);
    headers.insert("Sec-Fetch-Mode", "cors".parse()?);
    headers.insert("Sec-Fetch-Site", "same-origin".parse()?);
    headers.insert("TE", "trailers".parse()?);
    headers.insert("Token", token.parse()?);

    // let json_string = serde_json::to_string(&transaction_query).unwrap();
    // let json: serde_json::Value  = serde_json::from_str(&json_string).unwrap();

    // println!("{:#?}", json);
    // println!("{:#?}", base_url.join(&endpoint).unwrap());
    
    let request = client.post(base_url.clone().join(&endpoint).unwrap())
        .headers(headers)
        .json(&transaction_query)
        .send()
        .await?
        .text()
        .await?;

    let cookies = jar.cookies(&base_url).unwrap();
    fs::write("cookies.txt", cookies.to_str().unwrap()).expect("Unable to write file");

    let json_object: Value = serde_json::from_str(&request.as_str())?;

    Ok(json_object)
}


