use crate::schema::voices;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = voices)]

pub struct Voice {
    pub id: i32,
    pub user_id: i32,
    pub voices_journal: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime
}

#[derive(Insertable, Deserialize, Selectable, QueryableByName)]
#[diesel(table_name = voices)]
pub struct NewVoiceLog {
    pub user_id: i32,
    pub voices_journal: String,
    pub created_at: NaiveDateTime
}

#[derive(Deserialize)]
pub struct NewVoiceLogInput {
    pub voices_journal: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = voices)]
pub struct UpdateVoices {
    pub voices_journal: Option<String>,
}