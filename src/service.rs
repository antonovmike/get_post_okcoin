use std::time::Duration;

#[allow(unused)]
use anyhow::{Result, anyhow};

use crate::client::*;

#[derive(Debug, Clone)]
pub struct Service<EC: ExchangeClient> {
    pub timeout: Duration,
    pub threshold: f64,
    pub address_1: String,
    pub address_2: String,
    pub exchange_client: EC,
}

impl<EC: ExchangeClient + std::marker::Sync> Service<EC> {
    pub fn new(timeout: Duration, threshold: f64, address_1: String, address_2: String, exchange_client: EC) -> Self {
        Self {
            timeout,
            threshold,
            address_1,
            address_2,
            exchange_client,
        }
    }

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
