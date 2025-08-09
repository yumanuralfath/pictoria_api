use crate::schema::{
    voices, voices_months, voices_months_voices, voices_weeks, voices_weeks_voices,
};
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
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Selectable, QueryableByName)]
#[diesel(table_name = voices)]
pub struct NewVoiceLog {
    pub user_id: i32,
    pub voices_journal: String,
    pub created_at: NaiveDateTime,
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

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = voices_weeks)]
pub struct VoicesWeeks {
    pub id: i32,
    pub user_id: i32,
    pub voices_week_journal: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Queryable, Identifiable, Associations, Insertable)]
#[diesel(table_name = voices_weeks_voices)]
#[diesel(belongs_to(VoicesWeeks, foreign_key = voices_week_id))]
#[diesel(belongs_to(Voice, foreign_key = voice_id))]
pub struct VoicesWeeksVoices {
    pub id: i32,
    pub voices_week_id: i32,
    pub voice_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = voices_weeks)]
pub struct NewVoicesWeeks {
    pub user_id: i32,
    pub voices_week_journal: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = voices_weeks_voices)]
pub struct NewVoicesWeeksVoices {
    pub voices_week_id: i32,
    pub voice_id: i32,
}

#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = voices_months)]
pub struct VoicesMonths {
    pub id: i32,
    pub user_id: i32,
    pub voices_month_journal: String,
    pub month: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = voices_months)]
pub struct NewVoicesMonths<'a> {
    pub user_id: i32,
    pub voices_month_journal: &'a str,
    pub month: &'a str,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = voices_months_voices)]
pub struct NewVoicesMonthsVoices {
    pub voices_month_id: i32,
    pub voice_id: i32,
}

