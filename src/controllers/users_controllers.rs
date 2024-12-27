use crate::models::users::{EditUser, LoginCredentials, NewUser, UpdatedUser, User};
use crate::output::user_output::{LoginResponse, PaginatedUserResponse, UserOutput};
use crate::services::users_services::UserService;
use crate::utils::db::DbPool;

pub struct UserController<'a> {
    service: UserService<'a>,
}

impl<'a> UserController<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        UserController {
            service: UserService::new(pool),
        }
    }

    pub fn get_users(&self, offset: i64, limit: i64, page: u32) -> PaginatedUserResponse {
        self.service.get_paginated_users(offset, limit, page)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Option<UserOutput> {
        self.service.get_user(user_id)
    }

    pub fn create_new_user(&self, new_user: NewUser) -> Result<User, String> {
        self.service.create_user(new_user)
    }

    pub fn login(&self, credentials: LoginCredentials) -> Option<LoginResponse> {
        self.service.login(credentials)
    }

    pub fn edit_user(&self, user_id: i32, user: EditUser) -> Result<User, String> {
        self.service.edit_user(user_id, user)
    }

    pub fn update_user(&self, user_id: i32, user: UpdatedUser) -> Result<User, String> {
        self.service.update_user(user_id, user)
    }
}
