
use serde::{Deserialize, Serialize};
use crate::output::pagination_output::PaginationInfo;

#[derive(Serialize, Deserialize)]
pub struct ChatOutput {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct PaginatedChatOutput {
    pub chats: Vec<ChatOutput>,
    pub pagination: PaginationInfo
}

#[derive(Serialize, Deserialize)]
pub struct CreateChatOutput {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct CreateChatResponse {
    pub success: bool,
    pub message: String,
}
impl CreateChatResponse {
    pub fn new(success: bool, message: String) -> Self {
        Self { success, message }
    }
}

