use crate::schema::log_books;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Selectable, Identifiable)]
#[diesel(table_name = log_books)]
#[diesel(belongs_to(User))]
pub struct LogBook {
    pub id: i32,
    pub date: NaiveDate,
    pub content: String,
    pub user_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = log_books)]
pub struct NewLogBook {
    pub content: String,
    pub date: NaiveDate,
    pub user_id: i32,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = log_books)]
pub struct UpdateLogBook {
    pub content: Option<String>,
    pub date: Option<NaiveDate>,
}
