use std::time::Duration;

#[allow(unused)]
use anyhow::{Result, anyhow};

use crate::client::*;

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
        log::info!("info");
        log::warn!("warning");
        log::error!("error");
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
    pub async fn run(&self) -> Result<()> {
        let mut account_counter = 2;

        loop {
            let current_balance = self.exchange_client.get_balance().await?;
            if current_balance > self.threshold {
                if account_counter == 2 {
                    self.exchange_client
                        .withdraw(current_balance, self.address_1.clone())
                        .await?;
                    log::trace!("test \t ADDRES 1");
                    account_counter = 1
                } else {
                    self.exchange_client
                        .withdraw(current_balance, self.address_2.clone())
                        .await?;
                    log::trace!("test \t ADDRES 2");
                    account_counter = 2
                }
            }

            std::thread::sleep(self.timeout);
        }
        #[allow(unused)]
        Ok(())
    }
}
