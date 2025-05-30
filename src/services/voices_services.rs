use crate::library::deepseek_ai::deepseek_chat;
use crate::models::voices::{NewVoiceLog, UpdateVoices, Voice, VoicesWeeks, NewVoicesWeeks, NewVoicesWeeksVoices};
use crate::schema::voices::dsl::*;
 use crate::schema::voices_weeks_voices::dsl::voices_weeks_voices;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use crate::utils::time_converter::{get_today_date, get_weekly_date};
use chrono::Utc;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde_json::Value;

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
    
        let date_now = get_today_date();
    
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

    fn get_weekly_voice_log(&self, auth_user: &AuthenticatedUser, date: NaiveDate) -> Result<Vec<Voice>, String> {
        let mut conn = self.get_connection();

        let weekly_dates =get_weekly_date(date);

        let weekly_date_min = weekly_dates.iter().min().unwrap();
        let weekly_date_max = weekly_dates.iter().max().unwrap();

    let response = voices
        .filter(user_id.eq(auth_user.user_id)) 
        .filter(created_at.between(weekly_date_min, weekly_date_max)) 
        .order(created_at.desc())
        .load::<Voice>(&mut conn)
        .map_err(|err| format!("Database error: {}", err));

    response
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

    pub fn get_voice_log_by_date(&self, _auth_user: AuthenticatedUser, date: NaiveDate) -> Result<Option<Voice>, String> {
        let mut conn = self.get_connection();

        let start_of_the_day = date.and_hms_opt(0, 0, 0).expect("invalid start of the day");
        let next_day_start = date.succ_opt()
                                                        .expect("Invalid next day start")
                                                        .and_hms_opt(0, 0, 0);

        voices.filter(created_at.ge(start_of_the_day))
              .filter(created_at.lt(next_day_start))
              .first::<Voice>(&mut conn)
              .optional()
              .map_err(|err| format!("DB error: {}", err))
    }
    
    pub async fn get_weekly_resume_voice(&self, auth_user: &AuthenticatedUser) -> Result<VoicesWeeks, String> {
        let mut conn = self.get_connection();

        let today = get_today_date();
        let weekly_voice_log = self.get_weekly_voice_log(auth_user, today)?;
        
        if weekly_voice_log.is_empty() {
            return Err("Tidak ada voice journal selama seminggu".to_string());
        }
        
        let weekly_voices_collect = weekly_voice_log
            .iter()
            .map(|y| y.voices_journal.trim())
            .collect::<Vec<_>>()
            .join("\n\n");

        let context_weekly = "Buatkan resume per poin dari apa saja yang telah disimpan dalam journal lalu berika kesimpulan mengenai apa saja yang telah di pelajari selama seminggu dan gambaran tentang si pembuat jurnal ini ".to_string();

        let response = deepseek_chat(weekly_voices_collect, context_weekly).await?;
        let voices_week_journal = response.get("content").and_then(Value::as_str).unwrap_or("").to_string();

        let now = Utc::now().naive_utc();

        let new_week_log = NewVoicesWeeks {
            user_id: auth_user.user_id,
            voices_week_journal,
            created_at: now,
            updated_at: now,
        };

        let inserted: VoicesWeeks = diesel::insert_into(crate::schema::voices_weeks::table)
            .values(&new_week_log)
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        let pivot_data: Vec<NewVoicesWeeksVoices> = weekly_voice_log
            .iter()
            .map(|v| NewVoicesWeeksVoices {
                voices_week_id: inserted.id,
                voice_id: v.id,
            })
            .collect();

        diesel::insert_into(voices_weeks_voices)
            .values(&pivot_data)
            .execute(&mut conn)
            .map_err(|e| format!("Insert pivot voices_weeks_voices failed: {}", e))?;

        Ok(inserted)
    }
    
}