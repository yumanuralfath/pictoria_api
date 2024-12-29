use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub thread_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub content: String,
    pub user_id: i32,
    pub thread_id: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = comments)]
pub struct UpdateComment {
    pub content: Option<String>,
}