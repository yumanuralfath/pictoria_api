use crate::library::deepseek_ai::deepseek_chat;
use crate::models::voices::{NewVoiceLog, NewVoicesWeeks, NewVoicesWeeksVoices, UpdateVoices, Voice, VoicesWeeks, NewVoicesMonths, VoicesMonths, NewVoicesMonthsVoices};
use crate::schema::voices::dsl as voices_dsl;
use crate::schema::voices::table as voices; 
use crate::schema::voices_weeks::dsl as weeks_dsl;
use crate::schema::voices_weeks::table as voices_weeks;
use crate::schema::voices_weeks_voices::dsl::voices_weeks_voices;
use crate::schema::voices_months::dsl as months_dsl;
use crate::schema::voices_months::table as voices_months;
use crate::schema::voices_months_voices::dsl::voices_months_voices;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use crate::utils::time_converter::{get_monthly_date, get_today_date, get_weekly_date};
use chrono::{DateTime, Utc, NaiveDate}; 
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
            .filter(voices_dsl::created_at.ge(date_now.and_hms_opt(0, 0, 0).unwrap()))
            .filter(voices_dsl::created_at.lt(date_now.succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap()))
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
        
        let weekly_dates = get_weekly_date(date);
        
        let weekly_date_min = weekly_dates.iter().min().unwrap();
        let weekly_date_max = weekly_dates.iter().max().unwrap();
        
        let response = voices
        .filter(voices_dsl::user_id.eq(auth_user.user_id)) 
        .filter(voices_dsl::created_at.between(weekly_date_min, weekly_date_max)) 
        .order(voices_dsl::created_at.desc())
        .load::<Voice>(&mut conn)
        .map_err(|err| format!("Database error: {}", err));
    
    response
    }  

    fn get_weekly_voice_today(&self) -> Result<Option<VoicesWeeks>, String> {
        let mut conn = self.get_connection();
        let today = get_today_date();

        voices_weeks
            .filter(weeks_dsl::created_at.ge(today.and_hms_opt(0, 0, 0).unwrap()))
            .filter(weeks_dsl::created_at.lt(today.succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap()))
            .first::<VoicesWeeks>(&mut conn)
            .optional()
            .map_err(|err| format!("DB error: {}", err))
        
    }


    fn get_monthly_voice_log(&self, auth_user: &AuthenticatedUser, date: NaiveDate) -> Result<Vec<Voice>, String> {
        let mut conn = self.get_connection();

        let monthly_dates = get_monthly_date(date);

        let monthly_date_min = monthly_dates.iter().min().unwrap();
        let monthly_date_max = monthly_dates.iter().max().unwrap();

        let response = voices
            .filter(voices_dsl::user_id.eq(auth_user.user_id))
            .filter(voices_dsl::created_at.between(monthly_date_min, monthly_date_max))
            .order(voices_dsl::created_at.desc())
            .load::<Voice>(&mut conn)
            .map_err(|err| format!("Database error: {}", err));

        response
    }

    fn get_monthly_voice_today(&self) -> Result<Option<VoicesMonths>, String> {
        let mut conn = self.get_connection();
        let today = get_today_date();

        voices_months
            .filter(months_dsl::created_at.ge(today.and_hms_opt(0, 0, 0).unwrap()))
            .filter(months_dsl::created_at.lt(today.succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap()))
            .first::<VoicesMonths>(&mut conn)
            .optional()
            .map_err(|err| format!("DB Error: {}", err))
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

        voices.filter(voices_dsl::created_at.ge(start_of_the_day))
              .filter(voices_dsl::created_at.lt(next_day_start))
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

        let weekly_voice_today = self.get_weekly_voice_today();

        match weekly_voice_today {
            Ok(Some(result)) => {
                return Ok(result);
            }
            Ok(None) => {
                // NEXT
            }
            Err(e) => {
                return Err(e);
            }
        }
        
        let weekly_voices_collect = weekly_voice_log
            .iter()
            .map(|y| {
                let tanggal_str = y.created_at
                    .map(|date| {
                        let date_utc: DateTime<Utc> = date.and_utc();

                        date_utc.format_localized("%A, %e %B %Y", chrono::Locale::id_ID).to_string()
                    })
                    .unwrap_or_else(|| "Tanggal tidak diketahui".to_string());

                format!("[Tanggal: {}]\nJurnal: {}", tanggal_str, y.voices_journal.trim())
            })
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        let context_weekly = r#"
            Kamu adalah seorang teman ngobrol yang suportif dan jago menganalisis cerita. Tugasmu adalah membaca kumpulan jurnal dari temanmu dan memberikan ringkasan yang hangat dan personal.

            Setiap catatan jurnal akan diawali dengan tanggalnya dalam format [Tanggal: ...]. Gunakan informasi tanggal ini untuk membuat ringkasanmu lebih hidup. Kamu bisa merujuk ke hari tertentu (misalnya "Di hari Senin," atau "Waktu tanggal 8 Juni, kamu...") atau menggunakan referensi waktu seperti "kemarin" atau "beberapa hari yang lalu" jika relevan.

            Gunakan bahasa yang santai, positif, dan langsung menyapa dengan kata "kamu". Buat seolah-olah kamu sedang membalas chat temanmu.

            Struktur respons kamu harus seperti ini:

            ### Kilas Balik Minggumu 🗓️
            Ceritakan kembali dalam 1-2 paragraf santai tentang apa saja yang sepertinya jadi highlight di minggu ini buat kamu. Sebutkan tema-tema utamanya, dan kaitkan dengan hari-hari saat itu terjadi.

            ### Insight Keren Minggu Ini ✨
            Sebutkan 2-3 hal keren atau pelajaran penting yang sepertinya kamu dapatkan minggu ini. Gunakan format poin yang singkat dan jelas.

            ### Sedikit Obrolan Tentang Kamu 🤔
            Berikan sedikit analisis personal tentang apa yang kamu rasakan atau pikirkan, berdasarkan apa yang kamu tulis. Fokus pada kekuatanmu atau hal-hal positif yang terlihat.

            Di akhir, berikan satu pertanyaan reflektif yang ringan untuk kamu pikirkan di minggu selanjutnya.
            "#.to_string();

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

    pub async fn get_monthly_resume_voice(&self, auth_user: &AuthenticatedUser) -> Result<VoicesMonths, String> {
        let mut conn = self.get_connection();

        let today = get_today_date();
        let monthly_voice_log = self.get_monthly_voice_log(auth_user, today)?;
        
        if monthly_voice_log.is_empty() {
            return Err("Tidak ada voice journal selama sebulan".to_string());
        }

        let monthly_voice_today = self.get_monthly_voice_today();

        match monthly_voice_today {
            Ok(Some(result)) => {
                return Ok(result);
            }
            Ok(None) => {
                // NEXT
            }
            Err(e) => {
                return Err(e);
            }
        }
        
        let monthly_voices_collect = monthly_voice_log
            .iter()
            .map(|y| {
                let tanggal_str = y.created_at
                    .map(|date| {
                        let date_utc: DateTime<Utc> = date.and_utc();
                        date_utc.format_localized("%A, %e %B %Y", chrono::Locale::id_ID).to_string()
                    })
                    .unwrap_or_else(|| "Tanggal tidak diketahui".to_string());

                format!("[Tanggal: {}]\nJurnal: {}", tanggal_str, y.voices_journal.trim())
            })
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");

        let context_monthly = r#"
            Kamu adalah seorang teman ngobrol yang suportif dan jago menganalisis cerita. Tugasmu adalah membaca kumpulan jurnal bulanan dari temanmu dan memberikan ringkasan yang hangat dan personal.

            Setiap catatan jurnal akan diawali dengan tanggalnya. Gunakan informasi tanggal ini untuk melihat tren atau tema besar selama sebulan.

            Gunakan bahasa yang santai, positif, dan langsung menyapa dengan kata "kamu".

            Struktur respons kamu harus seperti ini:

            ### Kilas Balik Bulanmu 🗓️
            Ceritakan kembali dalam 2-3 paragraf santai tentang tema besar, pertumbuhan, atau tantangan yang paling menonjol selama sebulan terakhir buat kamu.

            ### Momen Paling Berkesan ✨
            Sebutkan 2-3 momen atau pembelajaran paling penting yang sepertinya kamu dapatkan bulan ini.

            ### Refleksi dan Arah ke Depan 🤔
            Berikan analisis personal tentang perkembanganmu bulan ini dan berikan satu pertanyaan reflektif untuk membantumu memikirkan bulan selanjutnya.
            "#.to_string();

        let response = deepseek_chat(monthly_voices_collect, context_monthly).await?;
        let voices_month_journal = response.get("content").and_then(Value::as_str).unwrap_or("").to_string();

        let now_naive = Utc::now().naive_utc();
        let month_str = today.format_localized("%B %Y", chrono::Locale::id_ID).to_string();

        let new_month_log = NewVoicesMonths {
            user_id: auth_user.user_id,
            voices_month_journal: &voices_month_journal,
            month: &month_str,
            created_at: Some(now_naive),
            updated_at: now_naive,
        };

        let inserted: VoicesMonths = diesel::insert_into(crate::schema::voices_months::table)
            .values(&new_month_log)
            .get_result(&mut conn)
            .map_err(|e| e.to_string())?;

        let pivot_data: Vec<NewVoicesMonthsVoices> = monthly_voice_log
            .iter()
            .map(|v| NewVoicesMonthsVoices {
                voices_month_id: inserted.id,
                voice_id: v.id,
            })
            .collect();


        diesel::insert_into(voices_months_voices)
            .values(&pivot_data)
            .execute(&mut conn)
            .map_err(|e| format!("Insert pivot voices_months_voices failed: {}", e))?;

        Ok(inserted)
    }
    
}