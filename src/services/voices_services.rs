use crate::models::voices::{NewVoiceLog, Voice};
use crate::schema::voices::dsl::*;
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

    pub fn create_voice_log(&self, new_voice_log: NewVoiceLog) -> Result<Voice, String> {
        let mut conn = self.get_connection();

        if let Some(existing_voice) = self.get_today_voice()? {
            return Err(format!(
                "Voice log already exists for today: {}",
                existing_voice.voices_journal
            ));
        }
    
        diesel::insert_into(voices)
            .values(&new_voice_log)
            .returning(Voice::as_select())
            .get_result::<Voice>(&mut conn)
            .map_err(|e| format!("error creating voice log: {}", e))
    }

}