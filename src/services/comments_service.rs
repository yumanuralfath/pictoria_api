use crate::models::comments::{Comment, NewComment, UpdateComment};
use crate::output::comment_output::{CommentOutput, PaginatedCommentResponse};
use crate::output::pagination_output::PaginationInfo;
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
            .map_err(|e| format!("Error checking thread existence: {e}"))?;

        if thread_exists.is_none() {
            return Err(format!("Thread with ID {thread_id_param} does not exist"));
        }
        new_comment.thread_id = Some(thread_id_param);

        if new_comment.user_id.is_none() {
            new_comment.user_id = Some(user.user_id);
        }

        diesel::insert_into(comments)
            .values(new_comment)
            .returning(Comment::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error creating comment: {e}"))
    }

    pub fn get_comments_by_thread_id(
        &self,
        thread_id_param: i32,
        offset: i64,
        limit: i64,
    ) -> Vec<Comment> {
        use crate::schema::comments;
        let mut conn = self.get_connection();
        comments::table
            .filter(comments::thread_id.eq(thread_id_param))
            .limit(limit)
            .offset(offset)
            .select(Comment::as_select())
            .load::<Comment>(&mut conn)
            .unwrap_or_default()
    }

    pub fn count_comments_by_thread(&self, thread_id_param: i32) -> i64 {
        use crate::schema::comments;
        let mut conn = self.get_connection();

        comments::table
            .filter(comments::thread_id.eq(thread_id_param))
            .count()
            .get_result(&mut conn)
            .unwrap_or(0)
    }

    pub fn get_paginated_comments_by_thread(
        &self,
        thread_id_param: i32,
        offset: i64,
        limit: i64,
        page: u32,
    ) -> PaginatedCommentResponse {
        let comment_list = self.get_comments_by_thread_id(thread_id_param, offset, limit);

        let modified_result = comment_list
            .into_iter()
            .map(CommentOutput::from_comment)
            .collect();

        PaginatedCommentResponse {
            comments: modified_result,
            pagination: PaginationInfo {
                current_page: page,
                limit: limit as u32,
                total_items: self.count_comments_by_thread(thread_id_param),
            },
        }
    }

    fn get_comment_by_id(&self, comment_id: i32) -> Option<Comment> {
        let mut conn = self.get_connection();
        comments
            .find(comment_id)
            .first::<Comment>(&mut conn)
            .optional()
            .unwrap()
    }

    pub fn update_comment(
        &self,
        comment_id: i32,
        update_comment: UpdateComment,
        auth_user: &AuthenticatedUser,
    ) -> Result<Comment, String> {
        let mut conn = self.get_connection();

        let thread = self
            .get_comment_by_id(comment_id)
            .ok_or_else(|| "Comment not found".to_string())?;

        if thread.user_id != auth_user.user_id {
            return Err("Unauthorized to update this comment".to_string());
        }

        diesel::update(comments.find(comment_id))
            .set(update_comment)
            .returning(Comment::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error updating comment: {e}"))
    }

    pub fn delete_comment(
        &self,
        comment_id: i32,
        auth_user: &AuthenticatedUser,
    ) -> Result<(), String> {
        let mut conn = self.get_connection();

        let thread = self
            .get_comment_by_id(comment_id)
            .ok_or_else(|| "Comment not found".to_string())?;

        if thread.user_id != auth_user.user_id {
            return Err("Unauthorized to delete this comment".to_string());
        }

        diesel::delete(comments.find(comment_id))
            .execute(&mut conn)
            .map_err(|e| format!("Error deleting comment: {e}"))
            .map(|_| ())
    }
}
