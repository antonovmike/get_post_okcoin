use async_trait::async_trait;
use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use serde_json::json;
use anyhow::Result;

/// Two structs for deserializing a JSON response containing balance data.
///
/// Properties:
///
/// * `current_balance`: `current_balance` is a field of type `String` in the `BalanseResponseData`
/// struct. It is deserialized from the JSON property `totalEq` using the `serde` attribute
/// `#[serde(rename = "totalEq")]`. This field represents the current balance of an account
#[derive(Debug, serde::Deserialize)]
struct BalanseResponseData {
    #[serde(rename = "totalEq")]
    current_balance: String,
}

#[derive(Debug, serde::Deserialize)]
struct BalanseResponse {
    data: Vec<BalanseResponseData>,
}

/// The OkCoinClient struct represents a client for interacting with the OkCoin API, with fields for API
/// key, passphrase, base URL, and secret.
///
/// Properties:
///
/// * `api_key`: The API key is a unique identifier that allows the OkCoinClient to access the OkCoin
/// API and perform various operations such as trading, checking account balances, and retrieving market
/// data.
/// * `passphrase`: The `passphrase` property is a string that is used as an additional security measure
/// for accessing the OkCoin API. It is a user-defined string that is used in combination with the API
/// key and secret to authenticate API requests.
/// * `base_url`: The `base_url` property is a string that represents the base URL of the OkCoin API. It
/// is used to construct the full URL for each API endpoint.
/// * `secret`: The `secret` property is a private key used for authentication and signing requests to
/// the OkCoin API. It should be kept secret and not shared with anyone.
#[derive(Debug, Clone)]
pub struct OkCoinClient {
    pub api_key: String,
    pub passphrase: String,
    pub secret: String,
}

impl OkCoinClient {
    const URL_BASE: &str = "https://www.okcoin.com";
    const URL_BALANCE: &str = "/api/v5/account/balance?ccy=STX";
    const URL_WITHDRAWAL: &str = "/api/v5/asset/withdrawal";

    pub fn new(api_key: String, passphrase: String, secret: String) -> Self {
        Self {
            api_key,
            passphrase,
            secret,
        }
    }
}

/// The `ExchangeClient` trait defines two asynchronous methods: `get_balance` and `withdraw`. These
/// methods are used to interact with a cryptocurrency exchange and retrieve the current balance or
/// withdraw funds from the exchange. The `async` keyword indicates that these methods are asynchronous
/// and will return a `Future` that can be awaited. The `Result` type is used to handle errors that may
/// occur during the execution of these methods. The `Box<dyn Error>` type is used to represent any type
/// of error that implements the `Error` trait, which allows for more flexibility in handling errors.
#[async_trait]
pub trait ExchangeClient {
    async fn get_balance(&self) -> Result<f64>;
    async fn withdraw(&self, current_balance: f64, address: String, ) -> Result<()>;
}

/// The above code is implementing the `ExchangeClient` trait for the `OkCoinClient` struct. It
/// defines two async functions: `get_balance` and `withdraw`.

#[async_trait]
impl ExchangeClient for OkCoinClient {
    /// This retrieves the current balance of a user's account from an exchange API using their access key,
    /// passphrase, and timestamp.
    ///
    /// Returns:
    ///
    /// This function is returning a `Result` with a `f64` value representing the current balance. The `f64`
    /// value is wrapped in an `Ok` variant if the function executes successfully, otherwise it returns a
    /// `Box<dyn Error>` wrapped in an `Err` variant.
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

        let response = client.execute(request).await?;

        let json = response.text().await?;
        let balance_response: BalanseResponse = serde_json::from_str(&json)?;
        log::info!("Balance response: {balance_response:#?}");

        let current_balance = balance_response.data[0].current_balance.parse::<f64>()?;
        log::info!("Current balance = {current_balance}");

        let current_balance = 200.4; // fake balance
        Ok(current_balance)
    }

    /// This function withdraws a specified amount of cryptocurrency to a specified address using the OKEx API
    ///
    /// Arguments:
    ///
    /// * `current_balance`: The current balance of the user's account from which they want to withdraw
    /// funds.
    /// * `address`: The destination address where the funds will be withdrawn to.
    ///
    /// Returns:
    ///
    /// a `Result` with an empty tuple `()` as the success value and a `Box` containing a `dyn Error` trait
    /// object as the error value.

    async fn withdraw(&self, current_balance: f64, address: String) -> Result<()> {
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