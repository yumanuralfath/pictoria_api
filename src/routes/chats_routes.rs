use crate::output::chat_output::CreateChatResponse;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use crate::{controllers::chats_controller::ChatController, models::chats::NewChatInput};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};

#[post("/chat/<receiver_id>", data = "<chat>")]
pub fn create_chat(
    auth: AuthenticatedUser,
    receiver_id: i32,
    chat: Json<NewChatInput>,
    pool: &State<DbPool>,
) -> Result<Json<CreateChatResponse>, (Status, Json<Value>)> {
    let chat_controller = ChatController::new(pool.inner());

    match chat_controller.create_chat(auth, chat.into_inner(), receiver_id) {
        Ok(chat) => {
            let response = CreateChatResponse::new(true, chat.message);
            Ok(Json(response))
        }
        Err(e) => Err((Status::BadRequest, Json(json!({"error": e})))),
    }
}
