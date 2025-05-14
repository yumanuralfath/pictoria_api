use crate::models::users::{EditUser, LoginCredentials, NewUser, UpdatedUser, User};
use crate::output::user_output::{LoginResponse, PaginatedUserResponse, UserOutput};
use crate::services::users_services::UserService;
use crate::utils::auth::AuthenticatedUser;
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

    pub fn get_users(
        &self,
        offset: i64,
        limit: i64,
        page: u32,
        auth_user: &AuthenticatedUser,
    ) -> Result<PaginatedUserResponse, String> {
        self.service
            .get_paginated_users(offset, limit, page, auth_user)
    }

    pub fn get_user_by_id(
        &self,
        user_id: i32,
        auth_user: &AuthenticatedUser,
    ) -> Option<UserOutput> {
        self.service.get_user(user_id, auth_user)
    }

    pub fn create_new_user(&self, new_user: NewUser) -> Result<User, String> {
        self.service.create_user(new_user)
    }

    pub fn login(&self, credentials: LoginCredentials) -> Option<LoginResponse> {
        self.service.login(credentials)
    }

    pub fn is_admin_user(
        &self,
        auth_user: &AuthenticatedUser
    ) -> Result<(), String> {
        self.service.is_admin_user(auth_user)
    }

    pub fn edit_user(
        &self,
        user_id: i32,
        user: EditUser,
        auth_user: &AuthenticatedUser,
    ) -> Result<User, String> {
        self.service.edit_user(user_id, user, auth_user)
    }

    pub fn delete_user(&self, user_id: i32, auth_user: &AuthenticatedUser) -> Result<(), String> {
        self.service.delete_user(user_id, auth_user)
    }

    pub fn update_user(&self, user_id: i32, user: UpdatedUser) -> Result<User, String> {
        self.service.update_user(user_id, user)
    }

    pub fn get_username_by_id(
        &self,
        user_id: i32,
        _auth_user: &AuthenticatedUser,
    ) -> Option<String> {
        self.service.get_user_name_by_id(user_id)
    }
}
