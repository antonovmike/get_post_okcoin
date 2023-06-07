use reqwest::Client;
// use serde_json::Value;
use base64::engine::{Engine, general_purpose};
use hmac_sha256::HMAC;

// const YOUR_ACCOUNT_ADDRESS: &str = "your_account_address";
// const RECIPIENT_ADDRESS_1: &str = "recipient_address_1";
// const RECIPIENT_ADDRESS_2: &str = "recipient_address_2";
// const AMOUNT: i64 = 1000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");
    let passphrase = dotenv::var("OKCOIN_PASS_PHRASE").expect("OKCOIN_PASS_PHRASE not found");

// https://www.okcoin.com/docs-v5/en/#rest-api-authentication-generating-an-apikey
    let client = Client::new();

    let url_1 = "https://www.okcoin.com";
    let url_2 = "/api/v5/account/balance?ccy=STX";

    let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
    let message = format!("{timestamp}GET{url_2}");
    let sign = general_purpose::STANDARD.encode(
        HMAC::mac(message, api_secret)
    );

    let request = client.get(format!("{url_1}{url_2}"))
        .header("OK-ACCESS-KEY", api_key)
        .header("OK-ACCESS-PASSPHRASE", passphrase)
        .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
        .header("Content-Type", "application/json")
        .header("OK-ACCESS-SIGN", sign)
        .build()?;

    let response = client.execute(request).await?;

    let json = response.text().await?;
    println!("{}", &json);
    let balance_response: BalanseResponse = serde_json::from_str(&json)?;
    // println!("{}", balance_response.total_eq);
    dbg!(balance_response);

    Ok(())
}
// serde_aux -> serde with String to u64
#[derive(Debug, serde::Deserialize)]
struct BalanseResponseData {
    #[serde(rename="totalEq")]
    total_eq: String,
}

#[derive(Debug, serde::Deserialize)]
struct BalanseResponse {
    code: String,
    data: Vec<BalanseResponseData>,
}