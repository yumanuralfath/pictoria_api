use serde::Serialize;
use chrono::NaiveDateTime;


#[derive(Serialize)]
pub struct SaveVoiceOutput {
    pub voice_journal: String,
    pub created_at: NaiveDateTime,
    pub status: String
}
