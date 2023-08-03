use async_trait::async_trait;
use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use serde_json::json;
use anyhow::Result;

#[derive(Debug, serde::Deserialize)]
struct BalanseResponseData {
    bal: String,
}

#[derive(Debug, serde::Deserialize)]
struct BalanseResponse {
    data: Vec<BalanseResponseData>,
}

#[derive(Debug, Clone)]
pub struct OkCoinClient {
    pub api_key: String,
    pub passphrase: String,
    pub secret: String,
}

impl OkCoinClient {
    const URL_BASE: &str = "https://www.okcoin.com";
    const URL_BALANCE: &str = "/api/v5/asset/balances";
    const URL_WITHDRAWAL: &str = "/api/v5/asset/withdrawal";

    pub fn new(api_key: String, passphrase: String, secret: String) -> Self {
        Self {
            api_key,
            passphrase,
            secret,
        }
    }
}

#[async_trait]
pub trait ExchangeClient {
    async fn get_balance(&self) -> Result<f64>;
    async fn withdraw(&self, current_balance: f64, address: String, ) -> Result<()>;
}

#[async_trait]
impl ExchangeClient for OkCoinClient {
    async fn get_balance(&self) -> Result<f64> {
        let client = Client::new();

        let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
        let message = format!("{timestamp}GET{}", Self::URL_BALANCE);
        let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &self.secret));

        let request = client
            .get(format!("{}{}", Self::URL_BASE, Self::URL_BALANCE))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
            .header("Content-Type", "application/json")
            .header("OK-ACCESS-SIGN", sign.clone())
            .build()?;

        dbg!(&request);

        let response = client.execute(request).await?;

        let json = response.text().await?;
        dbg!(&json);

        let balance_response: BalanseResponse = serde_json::from_str(&json)?;
        log::info!("Balance response: {balance_response:#?}");

        let current_balance = balance_response.data[0].bal.parse::<f64>()?;
        log::info!("Current balance = {current_balance}");

        Ok(current_balance)
    }

    async fn withdraw(&self, current_balance: f64, address: String) -> Result<()> {
        let client = Client::new();

        let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());

        let body = json!({
            "amt": current_balance,
            "fee":"0.0005",
            "dest":"3",
            "ccy":"STX",
            "chain":"STX-STX",
            "toAddr": address
        });

        let message = format!("{timestamp}POST{}{body}", Self::URL_WITHDRAWAL);
        let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &self.secret));

        let request = client
            .post(format!("{}{}", Self::URL_BASE, Self::URL_WITHDRAWAL))
            .header("accept", "application/json")
            .header("CONTENT-TYPE", "application/json")
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-SIGN", sign)
            .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .body(body.to_string())
            .build()?;

        let response = client.execute(request).await?;

        let json = response.text().await?;

        log::info!("POST: {}", &json);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use anyhow::anyhow;

    use crate::service::Service;

    use super::*;

    struct MockingClient {
        balance: f64,
        withdraw_success: bool,
    }
    #[async_trait]
    impl ExchangeClient for MockingClient {
        async fn get_balance(&self) -> Result<f64> {
            Ok(self.balance)
        }
        #[allow(unused)]
        async fn withdraw(
            &self,
            current_balance: f64,
            address: String,
        ) -> Result<()> {
            if self.withdraw_success {
                Ok(())
            } else {
                Err(anyhow!("Withdrawal failed"))
            }
        }
    }

    #[tokio::test]
    async fn success() -> Result<()> {
        let exchange_client = MockingClient {
            balance: 100.1,
            withdraw_success: true,
        };
        let service = Service::new(Duration::from_secs(3), 0.0, String::new(), String::new(), exchange_client);
        service.run().await.expect("Success!");
        Ok(())
    } // FIX IT

    #[tokio::test]
    async fn withdraw_fail() -> Result<()> {
        let exchange_client = MockingClient {
            balance: 100.0,
            withdraw_success: false,
        };
        let service = Service::new(Duration::from_secs(3), 0.0, String::new(), String::new(), exchange_client);
        service.run().await.expect_err("Withdraw failed!");
        Ok(())
    }
}