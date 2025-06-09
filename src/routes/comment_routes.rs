use crate::controllers::comments_controllers::CommentController;
use crate::models::comments::{NewComment, UpdateComment};
use crate::output::comment_output::{CreateCommentResponse, PaginatedCommentResponse};
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use crate::utils::pagination::paginate;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use serde_json::Value;

#[post("/comment/<thread_id>", data = "<comment>")]
pub async fn create_comment(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    thread_id: i32,
    comment: Json<NewComment>,
) -> Result<Json<CreateCommentResponse>, (Status, Json<Value>)> {
    let comment_controller = CommentController::new(pool.inner());

    match comment_controller.create_comment(auth, comment.into_inner(), thread_id) {
        Ok(comment) => {
            let response = CreateCommentResponse::from_create_comment(comment);
            Ok(Json(response))
        }
        Err(e) => Err((Status::BadRequest, Json(json!({"error": e})))),
    }
}

#[get("/comments/<thread_id>?<page>&<limit>")]
pub async fn get_comments(
    thread_id: i32,
    page: Option<u32>,
    limit: Option<u32>,
    pool: &State<DbPool>,
) -> Json<PaginatedCommentResponse> {
    let controller = CommentController::new(pool.inner());
    let (offset, limit_val) = paginate(page, limit);

    Json(controller.get_paginated_comments_by_thread(
        thread_id,
        offset,
        limit_val,
        page.unwrap_or(1)
    ))
}

#[get("/comment/<thread_id>")]
pub async fn get_number_comments_by_thread(
    _auth: AuthenticatedUser,
    thread_id: i32,
    pool: &State<DbPool>,
) -> Json<i64> {
    let controller = CommentController::new(pool.inner());
    Json(controller.get_number_comments_by_thread(thread_id))
}

#[put("/comment/<comment_id>", data = "<comment>")]
pub async fn update_comment(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    comment_id: i32,
    comment: Json<UpdateComment>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let comment_controller = CommentController::new(pool.inner());
    match comment_controller.update_comment(comment_id, comment.into_inner(), &auth) {
        Ok(updated_comment) => Ok(Json(json!({
            "message": "comment Update successfully",
            "comment": updated_comment
        }))),
        Err(e) => Err((
            Status::BadRequest,
            Json(json!({
                "error": e
            })),
        )),
    }
}

#[delete("/comment/<comment_id>")]
pub async fn delete_comment(
    auth: AuthenticatedUser,
    pool: &State<DbPool>,
    comment_id: i32,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let comment_controller = CommentController::new(pool.inner());
    match comment_controller.delete_comment(comment_id, &auth) {
        Ok(_) => Ok(Json(json!({
            "message": "comment deleted successfully",
        }))),
        Err(e) => Err((
            Status::BadRequest,
            Json(json!({
                "error": e
            })),
        )),
    }
}
