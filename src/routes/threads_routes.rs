use crate::controllers::threads_controllers::ThreadController;
use crate::models::threads::{NewThread, UpdateThread};
use crate::output::thread_output::{CreateThreadResponse, PaginatedThreadResponse};
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use crate::utils::pagination::paginate;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use serde_json::Value;
use validator::Validate;

#[get("/threads?<page>&<limit>")]
pub async fn get_threads(
    _auth: AuthenticatedUser,
    page: Option<u32>,
    limit: Option<u32>,
    pool: &State<DbPool>,
) -> Json<PaginatedThreadResponse> {
    let controller = ThreadController::new(pool.inner());
    let (offset, limit_val) = paginate(page, limit);

    Json(controller.get_paginated_threads(offset, limit_val, page.unwrap_or(1), &_auth))
}

#[post("/thread", data = "<thread>")]
pub async fn create_thread(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    thread: Json<NewThread>,
) -> Result<Json<CreateThreadResponse>, (Status, Json<Value>)> {
    let thread_controller = ThreadController::new(pool.inner());

    let new_thread = thread.into_inner();

    if let Err(validation_errors) = new_thread.validate() {
        return Err((
            Status::UnprocessableEntity,
            Json(json!({"error": validation_errors})),
        ));
    }

    match thread_controller.create_thread(new_thread, &auth) {
        Ok(thread) => {
            let response = CreateThreadResponse::from_create_thread(thread);
            Ok(Json(response))
        }
        Err(e) => Err((Status::BadRequest, Json(json!({"error": e})))),
    }
}

#[put("/thread/<id>", data = "<thread>")]
pub async fn update_thread(
    id: i32,
    auth: AuthenticatedUser,
    thread: Json<UpdateThread>,
    pool: &State<DbPool>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let thread_controller = ThreadController::new(pool.inner());
    match thread_controller.edit_thread(id, thread.into_inner(), &auth) {
        Ok(updated_thread) => Ok(Json(json!({
            "message": "thread Update successfully",
            "thread": updated_thread
        }))),
        Err(e) => Err((
            Status::BadRequest,
            Json(json!({
                "error": e
            })),
        )),
    }
}

#[delete("/thread/<id>")]
pub async fn delete_thread(
    id: i32,
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let thread_controller = ThreadController::new(pool.inner());
    match thread_controller.delete_thread(id, &auth) {
        Ok(_) => Ok(Json(json!({
            "message": "thread deleted successfully",
        }))),
        Err(e) => Err((
            Status::BadRequest,
            Json(json!({
                "error": e
            })),
        )),
    }
}
