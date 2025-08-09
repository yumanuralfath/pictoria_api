use reqwest::Client;
use serde_json::{json, Value};
use std::net::IpAddr;

use crate::library::base_lib_key::{
    WEBHOOK_IP_BAN_PASSWORD, WEBHOOK_IP_BAN_URL, WEBHOOK_IP_BAN_USER,
};

pub async fn send_block_webhook(ip: IpAddr, payload: Option<Value>) {
    let client = Client::new();
    let url = WEBHOOK_IP_BAN_URL.to_string();

    let default_payload = json!({
        "ip": ip.to_string(),
        "reason": "abuse detected from rate limiter"
    });

    let body = payload.unwrap_or(default_payload);

    let res = client
        .post(url)
        .basic_auth(
            WEBHOOK_IP_BAN_USER.to_string(),
            Some(WEBHOOK_IP_BAN_PASSWORD.to_string()),
        )
        .json(&body)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                println!("Webhook sent successfully for IP {ip}");
            } else {
                println!("Webhook failed: {}", response.status());
            }
        }
        Err(err) => {
            println!("Error sending webhook: {err}");
        }
    }
}
