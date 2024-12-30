use crate::models::comments::Comment;
use crate::output::pagination_output::PaginationInfo;
use crate::utils::time_converter::convert_to_wib;

use serde::Serialize;

#[derive(Serialize)]
pub struct CommentOutput {
    pub id: i32,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub thread_id: i32,
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct PaginatedCommentResponse {
    pub threads: Vec<CommentOutput>,
    pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct CreateCommentResponse {
    pub id: i32,
    pub message: String,
}

impl CommentOutput {
    pub fn from_comment(comment: Comment) -> Self {
        let created_at_wib = convert_to_wib(comment.created_at);
        let updated_at_wib = convert_to_wib(comment.updated_at);

        CommentOutput {
            id: comment.id,
            content: comment.content,
            user_id: comment.user_id,
            thread_id: comment.thread_id,
            created_at: created_at_wib,
            updated_at: updated_at_wib,
        }
    }
}

impl CreateCommentResponse {
    pub fn from_create_comment(comment: Comment) -> Self {
        CreateCommentResponse {
            id: comment.id,
            message: "comment uploaded successfully".to_string(),
        }
    }
}
