use chrono::NaiveDate;
use crate::models::voices::{NewVoiceLog, NewVoiceLogInput, UpdateVoices, Voice, VoicesMonths, VoicesWeeks};
use crate::services::voices_services::VoiceServices;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;

pub struct VoiceController<'a> {
    service: VoiceServices<'a>,
}

impl <'a> VoiceController<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        VoiceController {
            service: VoiceServices::new(pool),
        }
    }

    pub fn save_voice(
        &self,
        user: AuthenticatedUser,
        voice_input: NewVoiceLogInput,
    ) -> Result<Voice, String> {
        let new_voice = NewVoiceLog {
            user_id: user.user_id,
            voices_journal: voice_input.voices_journal,
            created_at: chrono::Utc::now().naive_utc(),
        };
    
        self.service.create_voice_log(new_voice, &user)
    }

    pub fn edit_voice_journal(
        &self,
        voice_log_id: i32,
        user: &AuthenticatedUser,
        voice_journal: UpdateVoices,
    ) -> Result<Voice, String> {
        self.service.update_voice_log(voice_log_id, voice_journal, user)
    }

    pub fn delete_voice(
        &self, 
        voice_log_id: i32,
        user: &AuthenticatedUser
    ) -> Result<(), String> {
        self.service.delete_voice_log(voice_log_id, user)
    }

    pub fn get_voice_log_by_date(
        &self,
        user: AuthenticatedUser,
        date: NaiveDate
    ) -> Result<Option<Voice>, String> {
        self.service.get_voice_log_by_date(user, date)
    }

    pub async fn get_weekly_resume_voice(
        &self,
        user: AuthenticatedUser
    ) -> Result<VoicesWeeks, String>{
        self.service.get_weekly_resume_voice(&user).await
    }

    pub async fn get_monthly_resume_voice(
        &self,
        user: AuthenticatedUser
    ) -> Result<VoicesMonths, String>{
        self.service.get_monthly_resume_voice(&user).await
    }

    pub async fn get_active_date_monthly(
        &self,
        user: &AuthenticatedUser,
        date: NaiveDate
    ) -> Result<Vec<NaiveDate>, String>{
        self.service.get_active_dates_in_month(user, date)
    }

}