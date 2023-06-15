use std::error::Error;
use std::time::Duration;

use async_trait::async_trait;
use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
// use serde_json::json;
// use tokio::time::Timeout;

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

#[derive(Debug, Clone)]
pub struct Service<EC: ExchangeClient> {
    pub timeout: Duration,
    pub threshold: f64,
    pub address: String,
    pub exchange_client: EC,
}

impl<EC: ExchangeClient + std::marker::Sync> Service<EC> {
    pub fn new(timeout: Duration, threshold: f64, address: String, exchange_client: EC) -> Self {
        Self {
            timeout,
            threshold,
            address,
            exchange_client,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.exchange_client.get_balance()? > Box::pin(self.threshold) {
            self.exchange_client.withdraw(self.address.clone())?
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
    fn withdraw(&self, _address: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub struct OkCoinClient {
    pub api_key: String,
    pub passphrase: String,
    pub url_base: String,
    pub secret: String,
}

impl OkCoinClient {
    pub fn new(api_key: String, passphrase: String, base_url: String, secret: String) -> Self {
        Self {
            api_key,
            passphrase,
            url_base: base_url,
            secret,
        }
    }
    fn timestamp() {
        todo!()
    }
}

impl ExchangeClient for OkCoinClient {
    fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
        let _ = self.api_key;
        let _ = self.passphrase;
        let _ = self.url_base;
        let _ = self.secret;
        Self::timestamp();
        Ok(0.0)
    }
    fn withdraw(&self, _address: String) -> Result<(), Box<dyn Error>> {
        let _ = self.api_key;
        let _ = self.passphrase;
        let _ = self.url_base;
        let _ = self.secret;
        Self::timestamp();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockingClient {
        balance: f64,
        withdraw_success: bool,
    }

    impl ExchangeClient for MockingClient {
        fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
            Ok(self.balance)
        }
        fn withdraw(&self, _address: String) -> Result<(), Box<dyn Error>> {
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

    #[test]
    fn success() {
        let exchange_client = MockingClient {
            balance: 100.0,
            withdraw_success: true,
        };
        let service = Service::new(
            Duration::from_secs(3), 0.0, String::new(), exchange_client
        );
        service.run().expect("Success!");
    }

    #[test]
    fn withdraw_fail() {
        let exchange_client = MockingClient {
            balance: 100.0,
            withdraw_success: false,
        };
        let service = Service::new(
            Duration::from_secs(3), 0.0, String::new(), exchange_client
        );
        service.run().expect_err("Withdraw failed!");
    }
}
