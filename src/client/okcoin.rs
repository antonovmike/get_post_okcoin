use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    str::FromStr,
};

use async_trait::async_trait;
use base64::engine::{general_purpose, Engine};
use hmac_sha256::HMAC;
use reqwest::Client;
use reqwest::Method;
// use reqwest::StatusCode;
use serde::{
    de::{DeserializeOwned, Deserializer, Visitor},
    Deserialize, Serialize,
};
use thiserror::Error;

use super::ExchangeClient;

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

trait Request: Serialize {
    const URL_PATH: &'static str;
    const HTTP_METHOD: Method;
    type Response: DeserializeOwned + Debug;
}

#[derive(Debug, Serialize)]
struct BalanceRequest {}
#[allow(unused)]
#[derive(Debug, Deserialize)]
struct BalanceResponse {
    #[serde[deserialize_with = "serde_from_str", rename = "uTime"]]
    u_time: u64,
    #[serde[deserialize_with = "serde_from_str", rename = "totalEq"]]
    total_eq: f64,
    details: Vec<BalanceDetailedInfo>,
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
struct BalanceDetailedInfo {
    #[serde[rename = "ccy"]]
    currency: String,
    #[serde[deserialize_with = "serde_from_str"]]
    eq: f64,
    #[serde[deserialize_with = "serde_from_str", rename = "cashBal"]]
    cash_balance: f64,
    #[serde[deserialize_with = "serde_from_str", rename = "uTime"]]
    u_time: u64,
    #[serde[deserialize_with = "serde_from_str", rename = "disEq"]]
    discount_eq: f64,
    #[serde[deserialize_with = "serde_from_str", rename = "availBal"]]
    available_balance: f64,
    #[serde[deserialize_with = "serde_from_str", rename = "frozenBal"]]
    frozen_balance: f64,
    #[serde[deserialize_with = "serde_from_str", rename = "ordFrozen"]]
    frozen_in_orders: f64,
    #[serde[deserialize_with = "serde_from_str", rename = "eqUsd"]]
    eq_usd: f64,
    #[serde[deserialize_with = "serde_from_str", rename = "stgyEq"]]
    strategy_eq: f64,
}

impl Request for BalanceRequest {
    const URL_PATH: &'static str = "asset/balances";
    const HTTP_METHOD: Method = Method::GET;
    type Response = BalanceResponse;
}

#[derive(Debug, Serialize)]
struct WithdrawalRequest {
    amount: f64, 
    address: String,
}

impl Request for WithdrawalRequest {
    const URL_PATH: &'static str = "asset/withdrawal";
    const HTTP_METHOD: Method = Method::POST;
    type Response = WithdrawalResponse;
}
#[allow(unused)]
#[derive(Debug, Deserialize)]
struct WithdrawalResponse {
    #[serde[deserialize_with = "serde_from_str", rename = "uTime"]]
    u_time: u64,
    #[serde[deserialize_with = "serde_from_str", rename = "totalEq"]]
    total_eq: f64,
    #[serde[deserialize_with = "serde_to_str"]]
    amt: String,
    #[serde[deserialize_with = "serde_from_str", rename = "wdId"]]
    wd_id: String,
    ccy: String,
    #[serde[deserialize_with = "serde_from_str", rename = "clientId"]]
    client_id: String,
    chain: String, // "STX-Bitcoin"
}

#[derive(Debug, Clone)]
pub struct OkCoinClient {
    api_key: String,
    passphrase: String,
    secret: String,
    client: Client,
}

impl OkCoinClient {
    // const URL_BASE: &str = "";
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
            .request(R::HTTP_METHOD, format!("https://www.okcoin.com/api/v5/{}", R::URL_PATH))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
            .header("Content-Type", "application/json")
            .header("OK-ACCESS-SIGN", sign.clone())
            .body(body_json)
            .build()?;

        dbg!(&request);

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

fn serde_to_str<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s)
}

#[async_trait]
impl ExchangeClient for OkCoinClient {
    type Err = OkCoinClientError;
    async fn get_balance(&self) -> Result<f64, Self::Err> {
        let resp = self.request(BalanceRequest {}).await?;

        log::debug!("Balance response: {resp:?}");

        let balance = resp
            .details
            .iter()
            .find(|d| d.currency == "STX")
            .map(|bdi| bdi.eq)
            .unwrap_or_default(); // or "STX-..."?

        Ok(balance)
    }

    async fn withdraw(&self, current_balance: f64, address: String) -> Result<(), Self::Err> {
        let _reqw = self.request(WithdrawalRequest {amount: current_balance, address: address}).await?;

        todo!()
    }
}
