use async_trait::async_trait;
use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use reqwest::Method;
use reqwest::StatusCode;
use serde::{
    de::{DeserializeOwned, Deserializer, Visitor},
    Deserialize, Serialize,
};
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    str::FromStr,
};
use thiserror::Error;

use super::ExchangeClient;

trait Request: Serialize {
    const URL_PATH: &'static str;
    const HTTP_METHOD: Method;
    type Response: DeserializeOwned + Debug;
}

#[derive(Debug, Serialize)]
struct BalanceRequest {}

#[derive(Debug, Deserialize)]
struct BalanceResponse {}

impl Request for BalanceRequest {
    const URL_PATH: &'static str = "/api/v5/account/balance";
    const HTTP_METHOD: Method = Method::GET;
    type Response = BalanceResponse;
}

#[derive(Debug, Clone)]
pub struct OkCoinClient {
    api_key: String,
    passphrase: String,
    secret: String,
    client: Client,
}

impl OkCoinClient {
    const URL_BASE: &str = "https://www.okcoin.com";
    // const URL_WITHDRAWAL: &str = "/api/v5/asset/withdrawal";

    pub fn new(api_key: String, passphrase: String, secret: String) -> Self {
        Self {
            api_key,
            passphrase,
            secret,
            client: Client::new(),
        }
    }

    async fn request<R: Request>(&self, request: R) -> Result<R::Response, OkCoinClientError> {
        #[derive(Debug, Deserialize)]
        struct RawResponse<T> {
            #[serde(deserialize_with = "serde_from_str")]
            code: u16,
            msg: String,
            data: Option<Vec<T>>,
        }

        let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
        let body_json = serde_json::to_string(&request)?;
        let message = format!("{timestamp}{}{}{}", R::HTTP_METHOD, R::URL_PATH, body_json);
        let sign = general_purpose::STANDARD.encode(HMAC::mac(message, &self.secret));

        let request = self
            .client
            .request(R::HTTP_METHOD, format!("{}{}", Self::URL_BASE, R::URL_PATH))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
            .header("Content-Type", "application/json")
            .header("OK-ACCESS-SIGN", sign.clone())
            .body(body_json)
            .build()?;

        let response = self.client.execute(request).await?;
        log::debug!("response: {response:?}");

        if !response.status().is_success() {
            let http_code = response.status();
            let err_msg = match response.json::<RawResponse<R::Response>>().await {
                Ok(rr) => format!(
                    "HTTP error: HTTP status code: {}, exchange code: {}; message: {}",
                    http_code, rr.code, rr.msg
                ),
                Err(_) => format!("HTTP status code {http_code}"),
            };

            return Err(OkCoinClientError::RequestFailed(err_msg));
        }

        let raw_response: RawResponse<R::Response> = response.json().await?;
        log::debug!("raw_response: {raw_response:?}");

        if raw_response.code != 0 {
            return Err(OkCoinClientError::ApiRequest(
                raw_response.code,
                raw_response.msg,
            ));
        }

        raw_response
            .data
            .ok_or(OkCoinClientError::EmptyResponse)
            .and_then(|x| x.into_iter().next().ok_or(OkCoinClientError::EmptyResponse))
    }
}

fn serde_from_str<'de, T, D, FE>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr<Err = FE>,
    FE: Display,
{
    struct SerdeFromStr<T>(PhantomData<T>);

    impl<'de, T, FE> Visitor<'de> for SerdeFromStr<T>
    where
        T: FromStr<Err = FE>,
        FE: Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> FmtResult {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: serde::de::Error,
        {
            T::from_str(value).map_err(|e| E::custom(format!("{e}")))
        }
    }

    deserializer.deserialize_any(SerdeFromStr(PhantomData))
}

#[async_trait]
impl ExchangeClient for OkCoinClient {
    type Err = OkCoinClientError;
    async fn get_balance(&self) -> Result<f64, Self::Err> {
        let resp = self.request(BalanceRequest {}).await?;

        todo!()
    }

    async fn withdraw(&self, current_balance: f64, address: String) -> Result<(), Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum OkCoinClientError {
    #[error("(de)serializing from/to json failed {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("reqwest error {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    RequestFailed(String),
    #[error("OkCoin API request finished with error: code {0}, message: \"{1}\"")]
    ApiRequest(u16, String),
    #[error("API request succeed but response is empty")]
    EmptyResponse,
}
