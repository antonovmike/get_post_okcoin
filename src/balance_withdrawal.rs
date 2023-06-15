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

pub struct Service<EC: ExchangeClient> {
    timeout: Duration,
    threshold: f64,
    address: String,
    exchange_client: EC,
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
        loop {
            if self.exchange_client.get_balance()? > self.threshold {
                self.exchange_client.withdraw(self.address)?
            }
            std::thread::sleep(self.timeout);
        }

        Ok(())
    }
}

trait ExchangeClient {
    fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
        todo!()
    }
    fn withdraw(&self, address: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub struct OkCoinClient {

}

impl OkCoinClient {
    pub fn new() -> Self {
        todo!()
    }
}

impl ExchangeClient for OkCoinClient {
    fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
        todo!()
    }
    fn withdraw(&self, address: String) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}