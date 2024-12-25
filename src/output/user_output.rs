use crate::models::users::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserOutput {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct PaginatedUserResponse {
    pub users: Vec<UserOutput>,
    pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct PaginationInfo {
    pub current_page: u32,
    pub limit: u32,
    pub total_items: i64,
}

impl UserOutput {
    pub fn from_user(user: User) -> Self {
        UserOutput {
            id: user.id,
            username: user.username,
            email: user.email,
            is_admin: user.is_admin,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
            message: "User retrieved successfully".to_string(),
        }
    }
}