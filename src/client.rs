use anyhow::Result;
use async_trait::async_trait;
use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

trait Request: Serialize {
    const URL_PATH: &'static str;
    const HTTP_METHOD: Method;
    type Response: DeserializeOwned;
}

#[derive(Debug, Serialize)]
struct BalanceRequest {}

#[derive(Debug, Deserialize)]
struct BalanceResponse {}

impl Request for BalanceRequest {
    const URL_PATH: &'static str = "/api/v5/account/balance";
    const HTTP_METHOD: Method = Method::GET;
    type Response = BalanceResponse;
}

#[derive(Debug, Clone)]
pub struct OkCoinClient {
    api_key: String,
    passphrase: String,
    secret: String,
    client: Client,
}

impl OkCoinClient {
    const URL_BASE: &str = "https://www.okcoin.com";
    // const URL_WITHDRAWAL: &str = "/api/v5/asset/withdrawal";

    pub fn new(api_key: String, passphrase: String, secret: String) -> Self {
        Self {
            api_key,
            passphrase,
            secret,
            client: Client::new(),
        }
    }

    async fn request<R: Request>(&self, request: R) -> Result<R::Response> {
        #[derive(Debug, Deserialize)]
        struct RawResponse {
            #[serde(deserialize_with = "serde_from_str")]
            code: u16,
            msg: String,
            // data: Vec<T>,
        }

        let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
        let body_json = serde_json::to_string(&request)?;
        let message = format!("{timestamp}{}{}{}", R::HTTP_METHOD, R::URL_PATH, body_json);
        let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &self.secret));

        let request = self
            .client
            .request(R::HTTP_METHOD, format!("{}{}", Self::URL_BASE, R::URL_PATH))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
            .header("Content-Type", "application/json")
            .header("OK-ACCESS-SIGN", sign.clone())
            .body(body_json)
            .build()?;

        let response = self.client.execute(request).await?;
        log::debug!("response: {response:?}");
        let raw_response: RawResponse = response.json().await?;
        dbg!(raw_response);

        todo!()
    }
}

#[async_trait]
pub trait ExchangeClient {
    async fn get_balance(&self) -> Result<f64>;
    async fn withdraw(&self, current_balance: f64, address: String) -> Result<()>;
}

#[async_trait]
impl ExchangeClient for OkCoinClient {
    async fn get_balance(&self) -> Result<f64> {
        let resp = self.request(BalanceRequest {}).await?;

        todo!()
    }

    async fn withdraw(&self, current_balance: f64, address: String) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use anyhow::anyhow;
    use std::time::Duration;

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
        async fn withdraw(&self, current_balance: f64, address: String) -> Result<()> {
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
        let service = Service::new(
            Duration::from_secs(3),
            0.0,
            String::new(),
            String::new(),
            exchange_client,
        );
        service.run().await.expect("Success!");
        Ok(())
    } // FIX IT

    #[tokio::test]
    async fn withdraw_fail() -> Result<()> {
        let exchange_client = MockingClient {
            balance: 100.0,
            withdraw_success: false,
        };
        let service = Service::new(
            Duration::from_secs(3),
            0.0,
            String::new(),
            String::new(),
            exchange_client,
        );
        service.run().await.expect_err("Withdraw failed!");
        Ok(())
    }
}

fn serde_from_str<'de, T, D, FE>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::de::Deserializer<'de>,
    T: std::str::FromStr<Err = FE>,
    FE: std::fmt::Display,
{
    struct SerdeFromStr<T>(core::marker::PhantomData<T>);

    impl<'de, T, FE> serde::de::Visitor<'de> for SerdeFromStr<T>
    where
        T: std::str::FromStr<Err = FE>,
        FE: std::fmt::Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            T::from_str(value).map_err(|e| E::custom(format!("{e}")))
        }
    }

    deserializer.deserialize_any(SerdeFromStr(core::marker::PhantomData))
}
