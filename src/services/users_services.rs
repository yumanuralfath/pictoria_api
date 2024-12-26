use crate::models::users::{EditUser, LoginCredentials, NewUser, UpdatedUser, User};
use crate::output::user_output::{
    LoginResponse, PaginatedUserResponse, PaginationInfo, UserOutput,
};
use crate::schema::users::dsl::*;
use crate::utils::auth::{generate_token, hash_password, verify_password};
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct UserService<'a> {
    pool: &'a DbPool,
}

impl<'a> UserService<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        UserService { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get DB connection")
    }

    pub fn get_paginated_users(&self, offset: i64, limit: i64, page: u32) -> PaginatedUserResponse {
        let users_list = self.get_users(offset, limit);
        let modified_results: Vec<UserOutput> =
            users_list.into_iter().map(UserOutput::from_user).collect();

        PaginatedUserResponse {
            users: modified_results,
            pagination: PaginationInfo {
                current_page: page,
                limit: limit as u32,
                total_items: self.count_users(),
            },
        }
    }

    fn get_users(&self, offset: i64, limit: i64) -> Vec<User> {
        let mut conn = self.get_connection();
        users
            .limit(limit)
            .offset(offset)
            .load::<User>(&mut conn)
            .expect("Error loading users")
    }

    pub fn get_user(&self, user_id: i32) -> Option<UserOutput> {
        let mut conn = self.get_connection();
        users
            .find(user_id)
            .first::<User>(&mut conn)
            .ok()
            .map(UserOutput::from_user)
    }

    pub fn create_user(&self, mut new_user: NewUser) -> Result<User, String> {
        if new_user.username.is_empty() || new_user.email.is_empty() || new_user.password.is_empty()
        {
            return Err("Username, email, and password are required.".to_string());
        }

        let mut conn = self.get_connection();
        new_user.password = hash_password(&new_user.password);

        diesel::insert_into(users)
            .values(new_user)
            .get_result(&mut conn)
            .map_err(|e| format!("Error creating user: {}", e))
    }

    pub fn login(&self, credentials: LoginCredentials) -> Option<LoginResponse> {
        let user = self.authenticate_user(credentials)?;
        let token = generate_token(user.id);

        Some(LoginResponse {
            token,
            user: UserOutput::from_user(user),
        })
    }

    fn authenticate_user(&self, credentials: LoginCredentials) -> Option<User> {
        let mut conn = self.get_connection();
        let user = users
            .filter(email.eq(credentials.email))
            .first::<User>(&mut conn)
            .ok()?;

        if verify_password(&credentials.password, &user.password) {
            Some(user)
        } else {
            None
        }
    }

    pub fn edit_user(&self, user_id: i32, mut user: EditUser) -> User {
        let mut conn = self.get_connection();

        if let Some(ref new_password) = user.password {
            user.password = Some(hash_password(new_password));
        }

        diesel::update(users.find(user_id))
            .set(user)
            .get_result::<User>(&mut conn)
            .expect("Error editing user")
    }

    pub fn update_user(&self, user_id: i32, mut user: UpdatedUser) -> User {
        let mut conn = self.get_connection();

        if let Some(ref new_password) = user.password {
            user.password = Some(hash_password(new_password));
        }

        diesel::update(users.find(user_id))
            .set(user)
            .get_result(&mut conn)
            .expect("Error updating user")
    }

    pub fn count_users(&self) -> i64 {
        let mut conn = self.get_connection();
        users.count().get_result(&mut conn).unwrap_or(0)
    }
}
