use reqwest::{self, cookie::{CookieStore, Jar}, Url};
use std::{error::Error, sync::Arc, fs};
use serde_json::Value;

use super::structs::TransactionQuery;

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