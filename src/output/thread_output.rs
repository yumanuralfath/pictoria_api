use crate::models::threads::Thread;
use crate::output::pagination_output::PaginationInfo;
use crate::utils::time_converter::convert_to_wib;

use serde::Serialize;

#[derive(Serialize)]
pub struct ThreadOutput {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct PaginatedThreadResponse {
    pub threads: Vec<ThreadOutput>,
    pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct CreateThreadResponse {
    pub id: i32,
    pub message: String,
}

impl ThreadOutput {
    pub fn from_thread(thread: Thread) -> Self {
        let created_at_wib = convert_to_wib(thread.created_at);
        let updated_at_wib = convert_to_wib(thread.updated_at);

        ThreadOutput {
            id: thread.id,
            content: thread.content,
            user_id: thread.user_id,
            created_at: created_at_wib,
            updated_at: updated_at_wib,
        }
    }
}

impl CreateThreadResponse {
    pub fn from_create_thread(thread: Thread) -> Self {
        CreateThreadResponse {
            id: thread.id,
            message: "thread uploaded successfully".to_string(),
        }
    }
}
