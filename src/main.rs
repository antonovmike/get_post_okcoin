use reqwest::Client;
use base64::engine::{Engine, general_purpose};
use hmac_sha256::HMAC;
use std::thread;
use std::time::Duration;

const AMOUNT: u64 = 0;

#[derive(Debug, serde::Deserialize)]
struct BalanseResponseData {
    #[serde(rename="totalEq")]
    total_eq: String,
}

#[derive(Debug, serde::Deserialize)]
struct BalanseResponse {
    // code: String,
    data: Vec<BalanseResponseData>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = dotenv::var("OKCOIN_API_KEY").expect("OKCOIN_API_KEY not found");
    let api_secret = dotenv::var("OKCOIN_API_SECRET").expect("OKCOIN_API_SECRET not found");
    let passphrase = dotenv::var("OKCOIN_PASS_PHRASE").expect("OKCOIN_PASS_PHRASE not found");

    let recipient_address_1 = "RECIPIENT_ADDRESS_1";
    let recipient_address_2 = "RECIPIENT_ADDRESS_2";

    let client = Client::new();

    let url_1 = "https://www.okcoin.com";
    let url_2 = "/api/v5/account/balance?ccy=STX";
    let url_3 = "/api/v5/asset/withdrawal?ccy=STX";

    let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
    let message = format!("{timestamp}GET{url_2}");
    let sign = general_purpose::STANDARD.encode(
        HMAC::mac(message, api_secret.clone())
    );

    let mut account_counter = 2;
    let mut address_sequence = recipient_address_1;

    loop {
        let request = client.get(format!("{url_1}{url_2}"))
            .header("OK-ACCESS-KEY", api_key.clone())
            .header("OK-ACCESS-PASSPHRASE", passphrase.clone())
            .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
            .header("Content-Type", "application/json")
            .header("OK-ACCESS-SIGN", sign.clone())
            .build()?;

        let response = client.execute(request).await?;

        let json = response.text().await?;
        let balance_response: BalanseResponse = serde_json::from_str(&json)?;
        // let code_num = balance_response.code.parse::<u8>()?;
        println!("{balance_response:#?}");
        let total_eq = balance_response.data[0].total_eq.parse::<u64>()?;

        if total_eq >= AMOUNT {
            if account_counter == 2 {
                address_sequence = recipient_address_1;
                account_counter = 1
            } else {
                address_sequence = recipient_address_2;
                account_counter = 2
            }

            let timestamp = humantime::format_rfc3339_millis(std::time::SystemTime::now());
            let message = format!("{timestamp}GET{url_3}");
            let sign = general_purpose::STANDARD.encode(
                HMAC::mac(message, api_secret.clone())
            );
            dbg!(&sign);
            let request = client.post(format!("{url_1}{url_3}"))
                .header("OK-ACCESS-KEY", api_key.clone())
                .header("OK-ACCESS-PASSPHRASE", passphrase.clone())
                .header("OK-ACCESS-TIMESTAMP", format!("{timestamp}"))
                .header("Content-Type", "application/json")
                .header("OK-ACCESS-SIGN", sign.clone())
                .header("amt", total_eq)
                .header("dest", 3) // 3: internal, 4: on chain
                .header("toAddr", address_sequence)
                .header("fee", 0)
                .build()?;

            let response = client.execute(request).await?;

            let json = response.text().await?;
            println!("POST: {}", &json);
        }

        thread::sleep(Duration::from_secs(3));
    }
    Ok(())
}

// fn withdrawal() {}