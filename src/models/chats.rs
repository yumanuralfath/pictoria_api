use crate::schema::chats;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(table_name = chats)]
pub struct Chat {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = chats)]
pub struct NewChat {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
}

#[derive(Deserialize)]
pub struct NewChatInput {
    pub message: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = chats)]
pub struct UpdateChat {
    pub message: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct PromptRequest {
    pub prompt: String,
}