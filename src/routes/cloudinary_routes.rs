use rocket::post;
use rocket::serde::json::Json;
use rocket::form::Form;
use crate::models::cloudinary::{CloudinaryResponse, UploadForm};
use crate::library::cloudinary::cloudinary_upload_image_profile;
use crate::utils::auth::AuthenticatedUser;

#[post("/upload-profile-pic", data = "<form>")]
pub async fn upload_profile_pic(form: Form<UploadForm<'_>>, user: AuthenticatedUser) -> Result<Json<CloudinaryResponse>, String> {
    cloudinary_upload_image_profile(form.into_inner(), user).await
}