use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, RunQueryDsl,
};
use crate::{
    models::log_books::{LogBook, NewLogBook, UpdateLogBook},
    schema::log_books::{self, dsl::*},
    utils::{auth::AuthenticatedUser, db::DbPool},
};

pub struct LogBookService<'a> {
    pool: &'a DbPool,
}

impl<'a> LogBookService<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        LogBookService { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("failed to get DB connection")
    }

    pub fn create_log_book(
        &self,
        user: AuthenticatedUser,
        mut new_log_book: NewLogBook,
    ) -> Result<LogBook, String> {
        let mut conn = self.get_connection();
        
        new_log_book.user_id = user.user_id;
        
        diesel::insert_into(log_books::table)
            .values(&new_log_book)
            .get_result::<LogBook>(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn get_log_books(&self, user: AuthenticatedUser) -> Result<Vec<LogBook>, String> {
        let mut conn = self.get_connection();
        log_books
            .filter(user_id.eq(user.user_id))
            .load::<LogBook>(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn get_log_book_by_id(&self, user: AuthenticatedUser, log_id: i32) -> Result<LogBook, String> {
        let mut conn = self.get_connection();
        log_books
            .filter(user_id.eq(user.user_id).and(id.eq(log_id)))
            .first::<LogBook>(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn update_log_book(
        &self,
        user: AuthenticatedUser,
        log_id: i32,
        log_update: UpdateLogBook,
    ) -> Result<LogBook, String> {
        let mut conn = self.get_connection();
        diesel::update(log_books.filter(user_id.eq(user.user_id).and(id.eq(log_id))))
            .set(&log_update)
            .get_result::<LogBook>(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn delete_log_book(&self, user: AuthenticatedUser, log_id: i32) -> Result<usize, String> {
        let mut conn = self.get_connection();
        diesel::delete(log_books.filter(user_id.eq(user.user_id).and(id.eq(log_id))))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}
