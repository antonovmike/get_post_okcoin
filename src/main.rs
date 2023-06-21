mod client;
mod service;

use std::{time::Duration, fs::File, io::Read};

use anyhow::{Result, anyhow};
use serde::Deserialize;

use client::OkCoinClient;
use service::Service;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(default="default_timeout", with="humantime_serde")]
    timeout: Duration,
    threshold: f64,
    address_1: String,
    address_2: String,
    api_key: String,
    secret: String,
    passphrase: String,
}

impl Config {
    fn from_file(path: &str) -> Result<Self> {
        let mut toml = String::new();
        File::open(path).map_err(|e| {
            log::error!("Failed open config file \"{path}\": {e}");
            anyhow!("Failed open config file \"{path}\": {e}")
        })?.read_to_string(&mut toml).map_err(|e| {
            log::error!("Failed to read config file \"{path}\": {e}");
            anyhow!("Failed to read config file \"{path}\": {e}")
        })?;
        toml::from_str(&toml).map_err(|e| {
            log::error!("config parse failed: {e}");
            anyhow!("config parse failed: {e}")
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::try_init().map_err(|e| anyhow!("logger setup error: {e}"))?;

    let config = Config::from_file("Config.toml")?;
    log::debug!("running with config: {config:?}");
    let okcoin_client = OkCoinClient::new(config.api_key, config.passphrase, config.secret);

    let service = Service::new(
        config.timeout, config.threshold, config.address_1, config.address_2, okcoin_client
    );

    service.run().await.map_err(|e| anyhow!("{e}"))
}

const fn default_timeout() -> Duration {
    Duration::from_secs(3)
}