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
}

impl Service {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
        }
    }

    fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            std::thread::sleep(self.timeout);
        }

        Ok(())
    }
}