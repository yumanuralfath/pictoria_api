use crate::models::comments::{Comment, NewComment};
use crate::output::comment_output::PaginatedCommentResponse;
use crate::services::comments_service::CommentService;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;

pub struct CommentController<'a> {
    service: CommentService<'a>,
}

impl<'a> CommentController<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        CommentController {
            service: CommentService::new(pool),
        }
    }

    pub fn create_comment(
        &self,
        user: AuthenticatedUser,
        comment: NewComment,
        thread_id: i32,
    ) -> Result<Comment, String> {
        self.service.create_comment(user, comment, thread_id)
    }

    pub fn get_paginated_comments_by_thread(
        &self,
        thread_id: i32,
        limit: i64,
        offset: i64,
        page: u32
    ) -> PaginatedCommentResponse {
        self.service
            .get_paginated_comments_by_thread(thread_id, limit, offset, page)
    }

    pub fn get_number_comments_by_thread(&self, thread_id: i32) -> i64 {
        self.service.count_comments_by_thread(thread_id)
    }

    pub fn update_comment(
        &self,
        comment_id: i32,
        update_comment: crate::models::comments::UpdateComment,
        auth_user: &AuthenticatedUser,
    ) -> Result<Comment, String> {
        self.service
            .update_comment(comment_id, update_comment, auth_user)
    }

    pub fn delete_comment(
        &self,
        comment_id: i32,
        auth_user: &AuthenticatedUser,
    ) -> Result<(), String> {
        self.service.delete_comment(comment_id, auth_user)
    }
}
