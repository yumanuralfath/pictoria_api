use crate::library::base_lib_key::OPENROUTER_API_KEY;
use reqwest::Client;
use rocket::serde::json::Json;
use serde_json::{json, Value};

pub async fn deepseek_chat(prompt: String, context: String) -> Result<Json<Value>, String> {
    let body = json!({
        "model": "deepseek/deepseek-chat-v3-0324:free",
        "messages": [
            { "role": "system", "content": format!("Use the following context to answer: {}", context) },
            { "role": "user", "content": prompt }
        ]
    });

    let client = Client::new();
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://yumana.my.id")
        .header("X-Title", "Yuma Nur Alfath Website Portofolio")
        .header(
            "Authorization",
            format!("Bearer {}", OPENROUTER_API_KEY.as_str()),
        )
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if response.status().is_success() {
        let json: Value = response
            .json()
            .await
            .map_err(|e| format!("Invalid JSON: {e}"))?;
        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Content not found in response")?;

        Ok(Json(json!({ "content": content })))
    } else {
        let status = response.status();
        let err_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("HTTP Error {status}: {err_text}"))
    }
}

