use crate::{
    models::log_books::{LogBook, NewLogBook, UpdateLogBook},
    output::{
        log_book_output::{LogBookOutput, PaginatedLogBookResponse},
        pagination_output::PaginationInfo,
    },
    schema::log_books::{self, dsl::*},
    utils::{auth::AuthenticatedUser, db::DbPool},
};
use chrono::Local;
use diesel::prelude::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection, RunQueryDsl,
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
        user.is_admin_user()?;
        let mut conn = self.get_connection();

        new_log_book.user_id = user.user_id;
        new_log_book.date = Local::now().naive_local().date();

        diesel::insert_into(log_books::table)
            .values(&new_log_book)
            .get_result::<LogBook>(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn get_log_books_by_user(&self, offset: i64, limit: i64) -> Result<Vec<LogBook>, String> {
        let mut conn = self.get_connection();

        let query = log_books
            .into_boxed()
            .order((date.asc(), id.asc()))
            .limit(limit)
            .offset(offset);

        query.load::<LogBook>(&mut conn).map_err(|e| e.to_string())
    }

    pub fn count_log_books(&self) -> i64 {
        let mut conn = self.get_connection();
        let query = log_books.into_boxed();

        query.count().get_result(&mut conn).unwrap_or(0)
    }

    pub fn get_paginated_log_books(
        &self,
        offset: i64,
        limit: i64,
        page: u32,
    ) -> PaginatedLogBookResponse {
        let log_book_list = self
            .get_log_books_by_user(offset, limit)
            .unwrap_or_default();

        let modified_result = log_book_list
            .into_iter()
            .map(LogBookOutput::from_log_book)
            .collect();

        PaginatedLogBookResponse {
            log_books: modified_result,
            pagination: PaginationInfo {
                current_page: page,
                limit: limit as u32,
                total_items: self.count_log_books(),
            },
        }
    }

    pub fn get_log_book_by_id(&self, log_id: i32) -> Result<LogBook, String> {
        let mut conn = self.get_connection();
        let mut query = log_books.into_boxed();

        query = query.filter(id.eq(log_id));

        query.first::<LogBook>(&mut conn).map_err(|e| e.to_string())
    }

    pub fn update_log_book(
        &self,
        user: AuthenticatedUser,
        log_id: i32,
        log_update: UpdateLogBook,
    ) -> Result<LogBook, String> {
        user.is_admin_user()?;
        let mut conn = self.get_connection();
        diesel::update(log_books.filter(id.eq(log_id)))
            .set(&log_update)
            .get_result::<LogBook>(&mut conn)
            .map_err(|e| e.to_string())
    }

    pub fn delete_log_book(&self, user: AuthenticatedUser, log_id: i32) -> Result<usize, String> {
        user.is_admin_user()?;
        let mut conn = self.get_connection();
        diesel::delete(log_books.filter(id.eq(log_id)))
            .execute(&mut conn)
            .map_err(|e| e.to_string())
    }
}
