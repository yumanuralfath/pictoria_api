use rocket::serde::json::Json;
use serde_json::Value;
use crate::{controllers::users_controllers::UserController, utils::db::DbPool};
use rocket::State;
use crate::{models::chats::PromptRequest, utils::auth::AuthenticatedUser};
use crate::utils::search_context::search_context_from_json;
use crate::library::deepseek_ai::deepseek_chat;

#[post("/generate", data = "<prompt>")]
pub async fn generate(
    auth: AuthenticatedUser,
    prompt: Json<PromptRequest>,
    pool: &State<DbPool>,
) -> Result<Json<Value>, String> {
    let user_controller = UserController::new(pool.inner());
    user_controller.is_active_user(&auth)?;

    let user_prompt = &prompt.prompt;
    let context = search_context_from_json(user_prompt);

    deepseek_chat(user_prompt.clone(), context).await
}