use rocket::serde::json::Json;
use serde_json::{json, Value};
use std::env;
use reqwest::Client;
use crate::{controllers::users_controllers::UserController, utils::db::DbPool};
use rocket::State;
use crate::{models::chats::PromptRequest, utils::auth::AuthenticatedUser};
use crate::utils::search_context::search_context_from_json;


#[post("/generate", data = "<prompt>")]
pub async fn generate(
    auth: AuthenticatedUser,
    prompt: Json<PromptRequest>,
    pool: &State<DbPool>,
) -> Result<Json<Value>, String> {

    let user_controller = UserController::new(pool.inner());
    user_controller.is_active_user(&auth)?;

    let openrouter_api_key = env::var("OPENROUTER_API_KEY")
        .map_err(|_| "OPENROUTER_API_KEY must be set".to_string())?;

    let user_prompt = &prompt.prompt;
    let context = search_context_from_json(&user_prompt);

    let body = json!({
        "model": "deepseek/deepseek-chat-v3-0324:free",
        "messages": [
            { "role": "system", "content": format!("Use the following context to answer: {}", context) },
            { "role": "user", "content": user_prompt }
        ]
    });

    let client = Client::new();
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://yumana.my.id")
        .header("X-Title", "Yuma Nur Alfath Website Portofolio")
        .header("Authorization", format!("Bearer {}", openrouter_api_key))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

        if response.status().is_success() {
            let json: Value = response.json().await.map_err(|e| e.to_string())?;
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
            Err(format!("HTTP Error {}: {}", status, err_text))
        }
}