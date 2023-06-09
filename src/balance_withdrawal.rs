use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;

use crate::constants::*;

#[derive(Debug, serde::Deserialize)]
struct BalanseResponseData {
    #[serde(rename = "totalEq")]
    current_balance: String,
}

#[derive(Debug, serde::Deserialize)]
struct BalanseResponse {
    #[allow(unused)]
    code: String,
    data: Vec<BalanseResponseData>,
}

async fn personal_data() -> Vec<String> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");
    let passphrase = dotenv::var("OKCOIN_PASS_PHRASE").expect("OKCOIN_PASS_PHRASE not found");

    let api_and_pass = vec![api_key, api_secret, passphrase];
    api_and_pass
}

pub async fn b_and_w() -> Result<u64, Box<dyn std::error::Error>> {
    let key_and_pass = personal_data().await;

    let client = Client::new();

    let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
    let message = format!("{timestamp}GET{URL_BALANCE}");
    let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &key_and_pass[1]));

    let request = client
        .get(format!("{URL_BASE}{URL_BALANCE}"))
        .header("OK-ACCESS-KEY", &key_and_pass[0])
        .header("OK-ACCESS-PASSPHRASE", &key_and_pass[2])
        .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
        .header("Content-Type", "application/json")
        .header("OK-ACCESS-SIGN", sign.clone())
        .build()?;

    let response = client.execute(request).await?;

    let json = response.text().await?;
    let balance_response: BalanseResponse = serde_json::from_str(&json)?;
    println!("{balance_response:#?}");

    let current_balance = balance_response.data[0].current_balance.parse::<u64>()?;

    Ok(current_balance)
}

pub async fn withdrawal(current_balance: u64, address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let key_and_pass = personal_data().await;

    let client = Client::new();

    let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
    let message = format!("{timestamp}GET{URL_WITHDRAWAL}");
    let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &key_and_pass[1]));

    dbg!(&sign);

    let request = client
        .post(format!("{URL_BASE}{URL_WITHDRAWAL}"))
        .header("OK-ACCESS-KEY", &key_and_pass[0])
        .header("OK-ACCESS-PASSPHRASE", &key_and_pass[2])
        .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
        .header("Content-Type", "application/json")
        .header("OK-ACCESS-SIGN", sign.clone())
        .header("amt", current_balance)
        .header("dest", 3) // 3: internal, 4: on chain
        .header("toAddr", address)
        .header("fee", 0)
        .build()?;

    let response = client.execute(request).await?;

    let json = response.text().await?;
    println!("POST: {}", &json);

    Ok(())
}
