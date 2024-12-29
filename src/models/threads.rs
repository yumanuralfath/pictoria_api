use crate::schema::threads;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = threads)]
pub struct Thread {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Selectable, Validate)]
#[diesel(table_name = threads)]
pub struct NewThread {
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
    #[validate(range(min = 1, message = "Invalid user ID"))]
    pub user_id: Option<i32>,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = threads)]
pub struct UpdateThread {
    pub content: Option<String>,
}
