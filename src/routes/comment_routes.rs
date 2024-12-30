use crate::controllers::comments_controllers::CommentController;
use crate::models::comments::{Comment, NewComment, UpdateComment};
use crate::output::comment_output::{
    CommentOutput, CreateCommentResponse, PaginatedCommentResponse,
};
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
