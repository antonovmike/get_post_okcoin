use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use serde_json::json;
use async_trait::async_trait;

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

mod error {
    pub enum Error {
        ApiError,
        HttpError,
        OtherFuckingError,
    }

    pub type Result<T, E = Error> = ::core::result::Result<T, E>;
}

use error::Result;
/// Client for a crypto exchange
#[async_trait]
pub trait XClient {
    /// Get balance of an attached account
    async fn get_balance(&self) -> Result<f64, Box<dyn std::error::Error>>;

    /// Withdraw funds to address specified
    async fn withdrawal(&self, current_balance: f64, address: String) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct Address {
    pub recipient_addr_1: String,
    pub recipient_addr_2: String,
}

/// Real OkClick exchange client
pub struct OkClick {
    pub access_key: String,
    pub passhphrase: String,
    pub base_url: String,
    pub http_client: reqwest::Client,
}

#[async_trait]
impl XClient for OkClick {
    async fn get_balance(&self) -> Result<f64, Box<dyn std::error::Error>> {
        // go to okclick api and return balance
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

    async fn withdrawal(&self, current_balance: f64, address: String) -> Result<(), Box<dyn std::error::Error>> {
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
}

async fn personal_data() -> Vec<String> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");
    let passphrase = dotenv::var("OKCOIN_PASS_PHRASE").expect("OKCOIN_PASS_PHRASE not found");

    let api_and_pass = vec![api_key, api_secret, passphrase];
    api_and_pass
}

#[cfg(test)]
mod tests {
    use super::XClient;
    use async_trait::async_trait;
    use crate::balance_withdrawal::Address;
    
    /// Mock exchange client FOR TESTING PURPOSES ONLY!!!
    struct MockExchange {
        balance: f64,
    }
    #[async_trait]
    impl XClient for MockExchange {
        async fn get_balance(&self) -> Result<f64, Box<dyn std::error::Error>> {
            Ok(self.balance)
        }

        async fn withdrawal(&self, _: f64, _: String) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }
}