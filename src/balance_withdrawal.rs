use std::error::Error;
use std::time::Duration;

use async_trait::async_trait;
use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use serde_json::json;

use anyhow::{Result, anyhow};

use crate::constants::*;

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

/// The `Service` type is a generic struct that contains fields for timeout duration, threshold value,
/// address string, and an exchange client.
///
/// Properties:
///
/// * `timeout`: `timeout` is a property of the `Service` struct that represents the maximum amount of
/// time that the service will wait for a response from the exchange client before timing out. It is of
/// type `Duration`, which is a struct that represents a length of time, such as seconds or
/// milliseconds.
/// * `threshold`: The `threshold` property is a floating-point number that represents the maximum
/// allowed difference between the expected and actual exchange rate. If the difference between the
/// expected and actual exchange rate exceeds this threshold, the service will consider the exchange
/// rate to be unreliable and will not perform the transaction.
/// * `address`: The `address` property is a `String` that represents the network address of the
/// service. It could be an IP address or a domain name.
/// * `exchange_client`: `exchange_client` is a generic type parameter `EC` that represents the exchange
/// client used by the service. It could be any type that implements the `ExchangeClient` trait. This
/// allows the service to be flexible and work with different exchange clients without having to modify
/// the code.
#[derive(Debug, Clone)]
pub struct Service<EC: ExchangeClient> {
    pub timeout: Duration,
    pub threshold: f64,
    pub address_1: String,
    pub address_2: String,
    pub exchange_client: EC,
}

impl<EC: ExchangeClient + std::marker::Sync> Service<EC> {
    /// This function creates a new service
    ///
    /// Arguments:
    ///
    /// * `timeout`: The duration of time after which a request to the service will time out if it has not
    /// been completed.
    /// * `threshold`: The `threshold` parameter is a floating-point number that represents the minimum
    /// price difference between the current market price and the target price for a trade to be executed.
    /// If the difference is less than the threshold, the trade will not be executed.
    /// * `address`: The `address` parameter is a `String` that represents the address of the service. It
    /// could be an IP address or a domain name.
    /// * `exchange_client`: `exchange_client` is a parameter of type `EC`, which is likely a struct or enum
    /// representing a client for interacting with a cryptocurrency exchange. It is passed as an argument to
    /// the `new` function and stored as a field in the struct being created. This suggests that the
    /// `Service` struct
    ///
    /// Returns:
    ///
    /// The `new` function is not returning anything. It is creating a new instance of a struct and
    /// initializing its fields with the provided arguments.
    pub fn new(timeout: Duration, threshold: f64, address_1: String, address_2: String, exchange_client: EC) -> Self {
        println!("New Service ceated");
        Self {
            timeout,
            threshold,
            address_1,
            address_2,
            exchange_client,
        }
    }


/// Thi function continuously checks the balance of an exchange client and withdraws funds if 
/// if the balance is above a certain threshold.
/// 
/// Returns:
/// 
/// a `Result` type with an empty `Ok` value and an error type of `Box<dyn Error>`. However, the code
/// after the loop is unreachable, so it will never be executed.
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut account_counter = 2;

        loop {
            if self.exchange_client.get_balance().await? > self.threshold {
                if account_counter == 2 {
                    self.exchange_client
                        .withdraw(0.0, self.address_1.clone())
                        .await?;
                    println!("test \t ADDRES 1");
                    account_counter = 1
                } else {
                    self.exchange_client
                        .withdraw(0.0, self.address_2.clone())
                        .await?;
                    println!("test \t ADDRES 2");
                    account_counter = 2
                }
            }

            std::thread::sleep(self.timeout);
        }
        #[allow(unused)]
        Ok(())
    }
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

/// The `ExchangeClient` trait defines two asynchronous methods: `get_balance` and `withdraw`. These
/// methods are used to interact with a cryptocurrency exchange and retrieve the current balance or
/// withdraw funds from the exchange. The `async` keyword indicates that these methods are asynchronous
/// and will return a `Future` that can be awaited. The `Result` type is used to handle errors that may
/// occur during the execution of these methods. The `Box<dyn Error>` type is used to represent any type
/// of error that implements the `Error` trait, which allows for more flexibility in handling errors.
#[async_trait]
pub trait ExchangeClient {
    async fn get_balance(&self) -> Result<f64> {
        Ok(270.0) // fake balance
    }
    async fn withdraw(
        &self,
        _current_balance: f64,
        _address: String,
    ) -> Result<()> {
        Ok(())
    }
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

/// This function retrieves personal data from an API using environment variables.
///
/// Returns:
///
/// A vector of strings containing the API key, API secret, and passphrase for the OKCoin API.
async fn personal_data() -> Vec<String> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");
    let passphrase = dotenv::var("OKCOIN_PASS_PHRASE").expect("OKCOIN_PASS_PHRASE not found");

    let api_and_pass = vec![api_key, api_secret, passphrase];
    api_and_pass
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
    async fn success() -> Result<()> { // FIT IT
        let exchange_client = MockingClient {
            balance: 100.0,
            withdraw_success: true,
        };
        let service = Service::new(Duration::from_secs(TIMEOUT), 0.0, String::new(), String::new(), exchange_client);
        service.run().await.expect("Success!");
        Ok(())
    }

    #[tokio::test]
    async fn withdraw_fail() -> Result<()> {
        let exchange_client = MockingClient {
            balance: 100.0,
            withdraw_success: false,
        };
        let service = Service::new(Duration::from_secs(TIMEOUT), 0.0, String::new(), String::new(), exchange_client);
        service.run().await.expect_err("Withdraw failed!");
        Ok(())
    }
}