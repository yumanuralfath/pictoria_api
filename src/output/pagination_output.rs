use serde::Serialize;

#[derive(Serialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub limit: u32,
    pub total_items: i64,
}
