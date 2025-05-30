use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::Value;
use serde_json::json;
use crate::controllers::voices_controllers::VoiceController;
use crate::models::voices::NewVoiceLogInput;
use crate::models::voices::UpdateVoices;
use crate::models::voices::VoicesWeeks;
use crate::output::voice_output::SaveVoiceOutput;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use crate::utils::time_converter::parse_param_date;

#[post("/voice", data = "<voice_input>")]
pub async fn save_voice(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    voice_input: Json<NewVoiceLogInput>,
) -> Result<Json<SaveVoiceOutput>, (Status, Json<Value>)> {
    let voice_controller = VoiceController::new(pool.inner());

    match voice_controller.save_voice(auth, voice_input.into_inner()) {
        Ok(voice) => {
            let response = SaveVoiceOutput {
                voice_journal: voice.voices_journal,
                created_at: voice.created_at.expect("created_at should be present"),
                status: "Voice log save successfully".to_string(),
            };
            Ok(Json(response))
        }
        Err(e) => Err((Status::BadRequest, Json(json!({ "error": e })))),
    }
}

#[put("/voice/<id>", data = "<voice_edit>")]
pub async fn update_voice(
    id: i32,
    auth: AuthenticatedUser,
    voice_edit: Json<UpdateVoices>,
    pool: &State<DbPool>, 
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let voice_controller = VoiceController::new(pool.inner());

    match voice_controller.edit_voice_journal(id, &auth, voice_edit.into_inner()) {
        Ok(update_voice) => Ok(Json(json!({
            "messsage": "voice log update successfully",
            "voice log": update_voice.voices_journal
        }))),
        Err(e) => Err((
            Status::BadRequest,
            Json(json!({
                "error": e
            })),
        )),
    }
}

#[delete("/voice/<id>")]
pub async fn delete_voice(
    id: i32,
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let voice_controller = VoiceController::new(pool.inner());

    match  voice_controller.delete_voice(id, &auth) {
        Ok(_) => Ok(Json(json!({
            "Message": "Voice log succesfully removed"
        }))),
        Err(e)  => Err((
            Status::BadRequest,
            Json(json!({
                    "error": e
                })),
        )),
    }
}

#[get("/voice/<date>")]
pub async fn get_voice_log_by_date(
    auth: AuthenticatedUser,
    date: String,
    pool: &State<DbPool>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let voice_controller = VoiceController::new(pool.inner());
    let date_naive = parse_param_date(date);

    match voice_controller.get_voice_log_by_date(auth, date_naive?) {
        Ok(Some(voice)) => Ok(Json(json!({
            "status": "success",
            "data": voice
        }))),
        Ok(None) => Ok(Json(json!({
            "status": "success",
            "data": null,
            "message": "No voice log found on this date"
        }))),
        Err(e) => Err((
            Status::InternalServerError,
            Json(json!({
                "status": "error",
                "message": e
            }))
        )),
    }
}

#[get("/voicesweeks")]
pub async fn get_weekly_resume_voice(
    auth: AuthenticatedUser,
    pool: &State<DbPool>
) -> Result<Json<VoicesWeeks>, Status> {
    let voice_controller = VoiceController::new(pool.inner());

    match voice_controller.get_weekly_resume_voice(auth).await {
        Ok(result) => Ok(Json(result)),
        Err(e) => {
            eprintln!("Error creating weekly resume: {}", e);
            Err(Status::InternalServerError)
        }
    }
}
