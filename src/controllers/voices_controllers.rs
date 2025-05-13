use crate::models::voices::{NewVoiceLog, NewVoiceLogInput, UpdateVoices, Voice};
use crate::schema::voices_months::voice_id;
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
    
        self.service.create_voice_log(new_voice)
    }

    pub fn edit_voice_journal(
        &self,
        voice_log_id: i32,
        user: &AuthenticatedUser,
        voice_journal: UpdateVoices,
    ) -> Result<Voice, String> {
        self.service.update_voice_log(voice_log_id, voice_journal, user)
    }
}