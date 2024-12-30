use crate::models::comments::{Comment, NewComment, UpdateComment};
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
}
