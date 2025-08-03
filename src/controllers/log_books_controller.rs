use crate::{
    models::log_books::{LogBook, NewLogBook, UpdateLogBook},
    output::log_book_output::PaginatedLogBookResponse,
    services::log_books_service::LogBookService,
    utils::{auth::AuthenticatedUser, db::DbPool},
};

pub struct LogBookController<'a> {
    service: LogBookService<'a>,
}

impl<'a> LogBookController<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        LogBookController {
            service: LogBookService::new(pool),
        }
    }

    pub fn create_log_book(
        &self,
        user: AuthenticatedUser,
        new_log_book: NewLogBook,
    ) -> Result<LogBook, String> {
        self.service.create_log_book(user, new_log_book)
    }

    pub fn get_paginated_log_books(
        &self,
        user: AuthenticatedUser,
        limit: i64,
        offset: i64,
        page: u32,
    ) -> PaginatedLogBookResponse {
        self.service
            .get_paginated_log_books(user, offset, limit, page)
    }

    pub fn get_log_book_by_id(
        &self,
        user: AuthenticatedUser,
        log_id: i32,
    ) -> Result<LogBook, String> {
        self.service.get_log_book_by_id(user, log_id)
    }

    pub fn update_log_book(
        &self,
        user: AuthenticatedUser,
        log_id: i32,
        log_update: UpdateLogBook,
    ) -> Result<LogBook, String> {
        self.service.update_log_book(user, log_id, log_update)
    }

    pub fn delete_log_book(&self, user: AuthenticatedUser, log_id: i32) -> Result<usize, String> {
        self.service.delete_log_book(user, log_id)
    }
}
