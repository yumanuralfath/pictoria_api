use crate::models::comments::{Comment, NewComment, UpdateComment};
use crate::schema::comments::dsl::*;
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

    pub fn create_comment(&self, new_comment: NewComment) -> Result<Comment, String> {
        let mut conn = self.get_connection();

        diesel::insert_into(comments)
            .values(new_comment)
            .returning(Comment::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error creating comment: {}", e))
    }

    pub fn get_thread_comments(&self, thread_id: i32) -> Vec<Comment> {
        let mut conn = self.get_connection();
        comments
            .filter(crate::schema::comments::thread_id.eq(thread_id))
            .select(Comment::as_select())
            .load::<Comment>(&mut conn)
            .unwrap_or_default()
    }
}
