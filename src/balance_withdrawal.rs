use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use serde_json::json;
// use mockito::{Server, Matcher};
use mockito::Matcher::PartialJsonString;

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

pub async fn balance() -> Result<f64, Box<dyn std::error::Error>> {
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

    let current_balance = balance_response.data[0].current_balance.parse::<f64>()?;

    Ok(current_balance)
}

pub async fn withdrawal(
    current_balance: f64,
    address: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_and_pass = personal_data().await;

    let client = Client::new();

    let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());

    let body = json!({
        "amt": current_balance,
        "fee":"0.0005",
        "dest":"3",
        "ccy":"BTC",
        "chain":"BTC-Bitcoin",
        "toAddr": address
    });

    let message = format!("{timestamp}POST{URL_WITHDRAWAL}{body}");
    let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &key_and_pass[1]));

    let request = client
        .post(format!("{URL_BASE}{URL_WITHDRAWAL}"))
        .header("accept", "application/json")
        .header("CONTENT-TYPE", "application/json")
        .header("OK-ACCESS-KEY", &key_and_pass[0])
        .header("OK-ACCESS-SIGN", sign)
        .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
        .header("OK-ACCESS-PASSPHRASE", &key_and_pass[2])
        .body(body.to_string())
        .build()?;

    let response = client.execute(request).await?;

    let json = response.text().await?;
    println!("POST: {}", &json);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_balance() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = mockito::Server::new();
        let key_and_pass = personal_data().await;

        let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
        let message = format!("{timestamp}GET{URL_BALANCE}");
        let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &key_and_pass[1]));

        let mock = server
            .mock("GET", format!("/{URL_BALANCE}").as_str())
            .with_status(200)
            .with_header("OK-ACCESS-KEY", &key_and_pass[0])
            .with_header("OK-ACCESS-PASSPHRASE", &key_and_pass[2])
            .with_header("OK-ACCESS-TIMESTAMP", &format!("{timestamp}"))
            .with_header("Content-Type", "application/json")
            .with_header("OK-ACCESS-SIGN", &sign)
            .create();

        let balance = balance().await?;
        assert_eq!(balance, 1000.0, "Expected balance is 1000");
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_withdrawal() -> Result<(), Box<dyn std::error::Error>> {
        let mut server = mockito::Server::new();
        let key_and_pass = personal_data().await;

        let body = PartialJsonString("{amt: 1000, fee: 0.0005, dest: 3, ccy: BTC, chain: BTC-Bitcoin, toAddr: \"0x1234567890123456789012345678901234567890\"}".to_string());

        let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
        let message = format!("{timestamp}POST{URL_WITHDRAWAL}{body}");
        let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &key_and_pass[1]));

        let mock = server
            .mock("POST", "/withdrawal")
            .match_body(body)
            .with_status(200)
            .with_header("accept", "application/json")
            .with_header("CONTENT-TYPE", "application/json")
            .with_header("OK-ACCESS-KEY", &key_and_pass[0])
            .with_header("OK-ACCESS-SIGN", &sign)
            .with_header("OK-ACCESS-TIMESTAMP", &format!("{timestamp}"))
            .with_header("OK-ACCESS-PASSPHRASE", &key_and_pass[2])
            .create();

        withdrawal(1001.0, RECIPIENT_ADDR_1).await?;

        mock.assert();

        Ok(())
    }
}
