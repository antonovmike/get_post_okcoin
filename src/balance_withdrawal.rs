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

pub struct Service {
    timeout: Duration,
    threshold: f64,
}

impl Service {
    pub fn new(timeout: Duration, threshold: f64) -> Self {
        Self {
            timeout,
            threshold,
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            std::thread::sleep(self.timeout);
        }

        Ok(())
    }
}

trait ExchangeClient {
    fn get_balance(&self) -> Result<f64, Box<dyn Error>>;
    fn withdraw(&self, address: String) -> Result<(), Box<dyn Error>>;
}