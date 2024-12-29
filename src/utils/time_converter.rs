use chrono::{FixedOffset, NaiveDateTime, TimeZone};

/// Fungsi untuk mengonversi `NaiveDateTime` ke waktu WIB (UTC+8) dalam format string
pub fn convert_to_wib(naive_datetime: NaiveDateTime) -> String {
    // Zona waktu UTC+8 (WIB)
    let wib_offset = FixedOffset::east_opt(8 * 3600).expect("Invalid offset for WIB");

    // Konversi waktu ke UTC+8 dan ubah menjadi string
    wib_offset
        .from_utc_datetime(&naive_datetime)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}
