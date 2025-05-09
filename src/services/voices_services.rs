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

    pub fn create_voice_log(&self, new_voice_log: NewVoiceLog) -> Result<Voice, String> {
        let mut conn = self.get_connection();
    
        diesel::insert_into(voices)
            .values(&new_voice_log)
            .returning(Voice::as_select())
            .get_result::<Voice>(&mut conn)
            .map_err(|e| format!("error creating voice log: {}", e))
    }

}