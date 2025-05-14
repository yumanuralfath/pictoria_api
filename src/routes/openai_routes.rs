use rocket::serde::json::Json;
use serde_json::Value;
use std::env;
use reqwest::Client;
use urlencoding::encode;
use crate::{controllers::users_controllers::UserController, utils::db::DbPool};
use rocket::State;
use crate::{models::chats::PromptRequest, utils::auth::AuthenticatedUser};


#[post("/generate", data = "<prompt>")]
pub async fn generate(
    auth: AuthenticatedUser,
    prompt: Json<PromptRequest>,
    pool: &State<DbPool> 
) -> Result<Json<Value>, String> {
    let user_controller = UserController::new(pool.inner());
    user_controller.is_admin_user(&auth)?;

    let prompt = &prompt.prompt;
    let open_ai_key = env::var("OPENAI_KEY").expect("open ai api key must be set");
    let model = "gpt-4-32k";

    let api_url = format!(
        "http://195.179.229.119/gpt/api.php?prompt={}&api_key={}&model={}",
        encode(prompt),
        encode(&open_ai_key),
        encode(model)
    );

    let client = Client::new();
    let response = client.get(&api_url).send().await.map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let text = response.text().await.map_err(|e| e.to_string())?;
        let json: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
        Ok(Json(json))
    } else {
        Err(format!("HTTP Error: {}", response.status()))
    }

}