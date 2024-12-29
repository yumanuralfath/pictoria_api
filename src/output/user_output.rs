use crate::models::users::User;
use crate::output::pagination_output::PaginationInfo;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserOutput {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub profile_picture_url: Option<String>,
}

#[derive(Serialize)]
pub struct PaginatedUserResponse {
    pub users: Vec<UserOutput>,
    pub pagination: PaginationInfo,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserOutput,
}

impl UserOutput {
    pub fn from_user(user: User) -> Self {
        UserOutput {
            id: user.id,
            username: user.username,
            email: user.email,
            is_admin: user.is_admin,
            profile_picture_url: user.profile_picture_url,
        }
    }
}

impl CreateUserResponse {
    pub fn from_create_user(user: User) -> Self {
        CreateUserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            message: "User created successfully".to_string(),
        }
    }
}
