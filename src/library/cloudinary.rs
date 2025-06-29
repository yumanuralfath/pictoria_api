use chrono::Utc;
use reqwest::Client;
use rocket::tokio::io::AsyncReadExt;
use rocket::{serde::json::Json};
use reqwest::multipart::{Form, Part};
use crate::models::cloudinary::{CloudinaryResponse, UploadForm};
use crate::library::base_lib_key::{CLOUDINARY_API_BASE_URL, CLOUDINARY_API_CLOUD_NAME, CLOUDINARY_API_KEY, CLOUDINARY_API_SECRET};
use crate::utils::auth::AuthenticatedUser;

pub const UPLOAD_PRESET: &[&'static str] = &["profile_pictures"];

pub  async fn cloudinary_upload_image_profile(form:  UploadForm<'_>, user: AuthenticatedUser) -> Result<Json<CloudinaryResponse>, String> {
    let timestamp = Utc::now().timestamp().to_string();

    // Read the file into a Vec<u8>
    let mut file_bytes = Vec::new();

    form.file.open().await
        .map_err(|e| format!("Failed to open file: {}", e))?
        .read_to_end(&mut file_bytes).await
        .map_err(|e| format!("Failed to read file to end: {}", e))?;

    // Create the multipart part from the byte vector
    let part = Part::bytes(file_bytes)
        .file_name(user.user_id.to_string())
        .mime_str("image/png")
        .unwrap();

    let client = Client::new();
    
    let form_data = Form::new()
                            .part("file", part)
                            .text("timestamp", timestamp)
                            .text("upload_preset", UPLOAD_PRESET[0])
                            .text("api_key", CLOUDINARY_API_KEY.to_string());

    let url_api = format!("{}/{}/image/upload", CLOUDINARY_API_BASE_URL.to_string(), CLOUDINARY_API_CLOUD_NAME.to_string());

    let response = client
                            .post(&url_api)
                            .multipart(form_data)
                            .basic_auth(CLOUDINARY_API_KEY.to_string(), Some(CLOUDINARY_API_SECRET.to_string()))
                            .send()
                            .await
                            .map_err(|e| format!("Failed to send request: {}", e))?;
    let status = response.status();

    if status == 200 {
        let json: CloudinaryResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse success response: {}", e))?;
        Ok(Json(json))
    } else {
        let cld_error = response
            .headers()
            .get("x-cld-error")
            .and_then(|val| val.to_str().ok())
            .unwrap_or("Unknown error from Cloudinary");

        Err(format!(
            "Upload failed with status {}: {}",
            status, cld_error
        ))
    }
}