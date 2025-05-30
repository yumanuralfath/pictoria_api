use chrono::{Duration, FixedOffset, NaiveDate, NaiveDateTime, TimeZone};
use rocket::{http::Status, serde::json::Json};
use serde_json::{json, Value};

/// Fungsi untuk mengonversi `NaiveDateTime` ke waktu WIB (UTC+7) dalam format string
pub fn convert_to_wib(naive_datetime: NaiveDateTime) -> String {
    // Zona waktu UTC+7 (WIB)
    let wib_offset = FixedOffset::east_opt(7 * 3600).expect("Invalid offset for WIB");

    // Konversi waktu ke UTC+7 dan ubah menjadi string
    wib_offset
        .from_utc_datetime(&naive_datetime)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}


//parse str to naivedate
pub fn parse_param_date(date: String) -> Result<NaiveDate, (Status, Json<Value>)> {
    match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
        Ok(d) => Ok(d),
        Err(_) => {
            return Err((
                Status::BadRequest,
                Json(json!({
                    "Status": "Error",
                    "message": "Invalid date format. Use YYYY-MM-DD."
                })),
            ));
        }
    }
}

pub fn get_today_date() -> NaiveDate {
    let today_date = chrono::Utc::now().naive_utc().date();
    today_date
}

//Get weekly date
pub fn get_weekly_date(date: NaiveDate) -> Vec<NaiveDateTime> {
    let weekly_dates = (0..7)
        .map(|i| date - Duration::days(i));

    let result = weekly_dates.into_iter().map(|d| d.and_hms_opt(0, 0, 0).unwrap()).collect();
    result
}