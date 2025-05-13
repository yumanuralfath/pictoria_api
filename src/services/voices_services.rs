use crate::models::voices::{NewVoiceLog, Voice, UpdateVoices};
use crate::schema::voices::dsl::*;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct VoiceServices<'a> {
    pool: &'a DbPool,
}

impl<'a> VoiceServices<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        VoiceServices { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get DB connection")
    }

    fn get_today_voice(&self) -> Result<Option<Voice>, String> {
        let mut conn = self.get_connection();
    
        let date_now = chrono::Utc::now().naive_utc().date(); 
    
        voices
            .filter(created_at.ge(date_now.and_hms_opt(0, 0, 0).unwrap()))
            .filter(created_at.lt(date_now.succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap()))
            .first::<Voice>(&mut conn)
            .optional()
            .map_err(|err| format!("DB error: {}", err))
    }

    fn get_voice_log_by_id(&self, voice_log_id: i32) -> Option<Voice> {
        let mut conn = self.get_connection();

        voices
        .find(voice_log_id)
        .select(Voice::as_select())
        .first(&mut conn)
        .ok()
    }

    fn get_owned_voice_log(&self, voice_log_id: i32, auth_user: &AuthenticatedUser) -> Result<Voice, String> {
        let voice_log = self
        .get_voice_log_by_id(voice_log_id)
        .ok_or_else(|| "Voice log not found".to_string())?;

    if voice_log.user_id != auth_user.user_id {
        return Err("Unauthorized access to this voice log".to_string());
    }

    Ok(voice_log)
    }

    pub fn create_voice_log(&self, new_voice_log: NewVoiceLog) -> Result<Voice, String> {
        let mut conn = self.get_connection();

        if let Some(existing_voice) = self.get_today_voice()? {
            return Err(format!(
                "Voice log already exists for today: ({}) {}, Please update or delete voice log",
                existing_voice.id,
                existing_voice.voices_journal
            ));
        }
    
        diesel::insert_into(voices)
            .values(&new_voice_log)
            .returning(Voice::as_select())
            .get_result::<Voice>(&mut conn)
            .map_err(|e| format!("error creating voice log: {}", e))
    }

    pub fn update_voice_log(&self, voice_log_id: i32, update_voice_log:UpdateVoices, auth_user: &AuthenticatedUser) -> Result<Voice, String>{
        let mut conn = self.get_connection();

        let _ = self.get_owned_voice_log(voice_log_id, auth_user)?;

        diesel::update(voices.find(voice_log_id))
            .set(update_voice_log)
            .returning(Voice::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error updating voice: {}", e))

    }

    pub fn delete_voice_log(&self, voice_log_id: i32, auth_user: &AuthenticatedUser) -> Result<(), String> {
        let mut conn = self.get_connection();

        let _ = self.get_owned_voice_log(voice_log_id, auth_user)?;
        
        diesel::delete(voices.find(voice_log_id))
            .execute(&mut conn)
            .map_err(|e| format!("Error deleting thread: {}", e))
            .map(|_| ())
    }
}