use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Insertable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String, 
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
}
