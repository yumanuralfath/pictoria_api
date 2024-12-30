use crate::models::comments::{Comment, NewComment, UpdateComment};
use crate::schema::comments::dsl::*;
use crate::schema::threads::dsl::{id as thread_id, threads};
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct CommentService<'a> {
    pool: &'a DbPool,
}

impl<'a> CommentService<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        CommentService { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get DB connection")
    }

    pub fn create_comment(
        &self,
        user: AuthenticatedUser,
        mut new_comment: NewComment,
        thread_id_param: i32,
    ) -> Result<Comment, String> {
        let mut conn = self.get_connection();

        let thread_exists = threads
            .filter(thread_id.eq(thread_id_param))
            .select(thread_id)
            .first::<i32>(&mut conn)
            .optional()
            .map_err(|e| format!("Error checking thread existence: {}", e))?;

        if thread_exists.is_none() {
            return Err(format!("Thread with ID {} does not exist", thread_id_param));
        }
        new_comment.thread_id = Some(thread_id_param);

        if new_comment.user_id.is_none() {
            new_comment.user_id = Some(user.user_id);
        }

        diesel::insert_into(comments)
            .values(new_comment)
            .returning(Comment::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error creating comment: {}", e))
    }
}
