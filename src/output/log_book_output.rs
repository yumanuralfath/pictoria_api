use crate::models::log_books::LogBook;
use crate::output::pagination_output::PaginationInfo;
use serde::Serialize;

#[derive(Serialize)]
pub struct LogBookOutput {
    pub id: i32,
    pub date: String,
    pub content: String,
    pub user_id: i32,
}

#[derive(Serialize)]
pub struct PaginatedLogBookResponse {
    pub log_books: Vec<LogBookOutput>,
    pub pagination: PaginationInfo,
}

impl LogBookOutput {
    pub fn from_log_book(log_book: LogBook) -> Self {
        LogBookOutput {
            id: log_book.id,
            date: log_book.date.to_string(),
            content: log_book.content,
            user_id: log_book.user_id,
        }
    }
}
