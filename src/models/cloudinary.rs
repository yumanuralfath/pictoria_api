use rocket::fs::TempFile;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(FromForm)]
pub struct UploadForm<'r> {
    pub file: TempFile<'r>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudinaryResponse {
    pub access_control: Vec<AccessControl>,
    pub asset_folder: String,
    pub asset_id: String,
    pub bytes: u64,
    pub context: Option<Context>,
    pub created_at: String,
    pub display_name: Option<String>,
    pub etag: String,
    pub format: String,
    pub height: u32,
    pub original_extension: Option<String>,
    pub original_filename: Option<String>,
    pub placeholder: bool,
    pub public_id: String,
    pub resource_type: String,
    pub secure_url: String,
    pub signature: String,
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    pub kind: String,
    pub url: String,
    pub version: u64,
    pub version_id: String,
    pub width: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessControl {
    pub access_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub custom: HashMap<String, String>,
}