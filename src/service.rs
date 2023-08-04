use std::time::Duration;
use tokio::time::sleep;

#[allow(unused)]
use anyhow::{anyhow, Result};

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
    pub fn new(
        timeout: Duration,
        threshold: f64,
        address_1: String,
        address_2: String,
        exchange_client: EC,
    ) -> Self {
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

    pub async fn run(&self) -> Result<()> {
        let mut account_counter = 2;

        loop {
            if self.exchange_client.get_balance().await? > self.threshold {
                if account_counter == 2 {
                    self.exchange_client
                        .withdraw(0.0, self.address_1.clone())
                        .await?;
                    log::trace!("test \t ADDRES 1");
                    account_counter = 1
                } else {
                    self.exchange_client
                        .withdraw(0.0, self.address_2.clone())
                        .await?;
                    log::trace!("test \t ADDRES 2");
                    account_counter = 2
                }
            }

            sleep(Duration::from_secs(3)).await;
        }
        #[allow(unused)]
        Ok(())
        // self.exchange_client.get_balance().await?;
        // Ok(())
    }
}

// #[cfg(test)]
// mod test {
//     use anyhow::anyhow;
//     use std::time::Duration;

//     use crate::service::Service;

//     use super::*;

//     struct MockingClient {
//         balance: f64,
//         withdraw_success: bool,
//     }
//     #[async_trait]
//     impl ExchangeClient for MockingClient {
//         async fn get_balance(&self) -> Result<f64> {
//             Ok(self.balance)
//         }
//         #[allow(unused)]
//         async fn withdraw(&self, current_balance: f64, address: String) -> Result<()> {
//             if self.withdraw_success {
//                 Ok(())
//             } else {
//                 Err(anyhow!("Withdrawal failed"))
//             }
//         }
//     }

//     #[tokio::test]
//     async fn success() -> Result<()> {
//         let exchange_client = MockingClient {
//             balance: 100.1,
//             withdraw_success: true,
//         };
//         let service = Service::new(
//             Duration::from_secs(3),
//             0.0,
//             String::new(),
//             String::new(),
//             exchange_client,
//         );
//         service.run().await.expect("Success!");
//         Ok(())
//     } // FIX IT

//     #[tokio::test]
//     async fn withdraw_fail() -> Result<()> {
//         let exchange_client = MockingClient {
//             balance: 100.0,
//             withdraw_success: false,
//         };
//         let service = Service::new(
//             Duration::from_secs(3),
//             0.0,
//             String::new(),
//             String::new(),
//             exchange_client,
//         );
//         service.run().await.expect_err("Withdraw failed!");
//         Ok(())
//     }
// }
