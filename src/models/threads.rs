use crate::schema::threads;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = threads)]
pub struct Thread {
    pub id: i32,
    pub content: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = threads)]
pub struct NewThread {
    pub content: String,
    pub user_id: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = threads)]
pub struct UpdateThread {
    pub content: Option<String>,
}
