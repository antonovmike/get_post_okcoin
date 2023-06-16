use std::error::Error;
use std::time::Duration;

use async_trait::async_trait;
use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use serde_json::json;

use crate::constants::*;

#[derive(Debug, serde::Deserialize)]
struct BalanseResponseData {
    #[serde(rename = "totalEq")]
    current_balance: String,
}

#[derive(Debug, serde::Deserialize)]
struct BalanseResponse {
    data: Vec<BalanseResponseData>,
}

#[derive(Debug, Clone)]
pub struct Service<EC: ExchangeClient> {
    pub timeout: Duration,
    pub threshold: f64,
    pub address: String,
    pub exchange_client: EC,
}

impl<EC: ExchangeClient + std::marker::Sync> Service<EC> {
    pub fn new(timeout: Duration, threshold: f64, address: String, exchange_client: EC) -> Self {
        println!("New Service ceated");
        Self {
            timeout,
            threshold,
            address,
            exchange_client,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.exchange_client.get_balance().await? > self.threshold {
            self.exchange_client.withdraw(0.0, self.address.clone()).await?
        }

        std::thread::sleep(self.timeout);

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

#[async_trait]
pub trait ExchangeClient {
    async fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
        Ok(270.0) // fake balance
    }
    async fn withdraw(&self, _current_balance: f64, _address: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct OkCoinClient {
    pub api_key: String,
    pub passphrase: String,
    pub base_url: String,
    pub secret: String,
}

impl OkCoinClient {
    pub fn new(api_key: String, passphrase: String, base_url: String, secret: String) -> Self {
        Self {
            api_key,
            passphrase,
            base_url,
            secret,
        }
    }
}

#[async_trait]
impl ExchangeClient for OkCoinClient {
    async fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
        println!("Get balance (ExchangeClient)");

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
        println!("Balance response: {balance_response:#?}");

        let current_balance = balance_response.data[0].current_balance.parse::<f64>()?;
        println!("Current balance = {current_balance}");

        let current_balance = 200.4; // fake balance
        Ok(current_balance)
    }

    async fn withdraw(&self, current_balance: f64, address: String) -> Result<(), Box<dyn Error>> {
        let key_and_pass = personal_data().await;
        let client = Client::new();
    
        let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
    
        let body = json!({
            "amt": current_balance,
            "fee":"0.0005",
            "dest":"3",
            "ccy":"STX",
            "chain":"STX-Bitcoin",
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

#[cfg(test)]
mod test {
    use super::*;

    struct MockingClient {
        balance: f64,
        #[allow(unused)]
        withdraw_success: bool,
    }
    #[async_trait]
    impl ExchangeClient for MockingClient {
        async fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
            Ok(self.balance)
        }
        #[allow(unused)]
        async fn withdraw(&self, current_balance: f64, address: String) -> Result<(), Box<dyn Error>> {
            if self.withdraw_success {
                Ok(())
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::AddrInUse,
                    "TEST".to_string(),
                )))
            }
        }
    }

    #[tokio::test]
    async fn success() -> Result<(), Box<dyn Error>> {
        let exchange_client = MockingClient {
            balance: 100.0,
            withdraw_success: true,
        };
        let service = Service::new(
            Duration::from_secs(3), 0.0, String::new(), exchange_client
        );
        service.run().await.expect("Success!");
        Ok(())
    }

    #[tokio::test]
    async fn withdraw_fail() -> Result<(), Box<dyn Error>> {
        let exchange_client = MockingClient {
            balance: 100.0,
            withdraw_success: false,
        };
        let service = Service::new(
            Duration::from_secs(3), 0.0, String::new(), exchange_client
        );
        service.run().await.expect_err("Withdraw failed!");
        Ok(())
    }
}
