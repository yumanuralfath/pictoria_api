use rocket::State;
use rocket::{http::Status, serde::json::Json};
use serde_json::{json, Value};

use crate::{
    controllers::log_books_controller::LogBookController,
    models::log_books::{LogBook, NewLogBook, UpdateLogBook},
    utils::{auth::AuthenticatedUser, db::DbPool},
};

#[post("/logs", data = "<new_log_book>")]
pub fn create_log_book(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    new_log_book: Json<NewLogBook>,
) -> Result<Json<LogBook>, (Status, Json<Value>)> {
    let controller = LogBookController::new(pool);
    match controller.create_log_book(auth, new_log_book.into_inner()) {
        Ok(log_book) => Ok(Json(log_book)),
        Err(e) => Err((Status::InternalServerError, Json(json!({ "error": e })))),
    }
}

#[get("/logs")]
pub fn get_log_books(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
) -> Result<Json<Vec<LogBook>>, (Status, Json<Value>)> {
    let controller = LogBookController::new(pool);
    match controller.get_log_books(auth) {
        Ok(log_books) => Ok(Json(log_books)),
        Err(e) => Err((Status::InternalServerError, Json(json!({ "error": e })))),
    }
}

#[get("/logs/<id>")]
pub fn get_log_book_by_id(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    id: i32,
) -> Result<Json<LogBook>, (Status, Json<Value>)> {
    let controller = LogBookController::new(pool);
    match controller.get_log_book_by_id(auth, id) {
        Ok(log_book) => Ok(Json(log_book)),
        Err(e) => Err((Status::NotFound, Json(json!({ "error": e })))),
    }
}

#[put("/logs/<id>", data = "<log_update>")]
pub fn update_log_book(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    id: i32,
    log_update: Json<UpdateLogBook>,
) -> Result<Json<LogBook>, (Status, Json<Value>)> {
    let controller = LogBookController::new(pool);
    match controller.update_log_book(auth, id, log_update.into_inner()) {
        Ok(log_book) => Ok(Json(log_book)),
        Err(e) => Err((Status::InternalServerError, Json(json!({ "error": e })))),
    }
}

#[delete("/logs/<id>")]
pub fn delete_log_book(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    id: i32,
) -> Result<Status, (Status, Json<Value>)> {
    let controller = LogBookController::new(pool);
    match controller.delete_log_book(auth, id) {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => Err((Status::InternalServerError, Json(json!({ "error": e })))),
    }
}
