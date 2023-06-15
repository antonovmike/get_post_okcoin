use std::error::Error;
use std::time::Duration;

use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
// use humantime::Duration;
use reqwest::Client;
// use serde::de::Error;
use serde_json::json;
use tokio::time::Timeout;

use crate::constants::*;

#[derive(Debug, Clone)]
pub struct Service<EC: ExchangeClient> {
    pub timeout: Duration,
    pub threshold: f64,
    pub address: String,
    pub exchange_client: EC,
}

impl<EC: ExchangeClient> Service<EC> {
    pub fn new(timeout: Duration, threshold: f64, address: String, exchange_client: EC) -> Self {
        Self {
            timeout,
            threshold,
            address,
            exchange_client,
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        if self.exchange_client.get_balance()? > self.threshold {
            self.exchange_client.withdraw(self.address.clone())?
        }

        std::thread::sleep(self.timeout);

        Ok(())
    }
}

pub trait ExchangeClient {
    fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
        todo!()
    }
    fn withdraw(&self, address: String) -> Result<(), Box<dyn Error>> {
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
    fn withdraw(&self, address: String) -> Result<(), Box<dyn Error>> {
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
        fn withdraw(&self, address: String) -> Result<(), Box<dyn Error>> {
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
}
