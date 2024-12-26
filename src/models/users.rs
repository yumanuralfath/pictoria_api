use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::AsChangeset;
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
    pub profile_picture_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug, Selectable)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
    pub profile_picture_url: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct EditUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_admin: Option<bool>,
}

#[derive(Deserialize, AsChangeset, Debug, Serialize)]
#[diesel(table_name = users)]
pub struct UpdatedUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub profile_picture_url: Option<String>,
}
