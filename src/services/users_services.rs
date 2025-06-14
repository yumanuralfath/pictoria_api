use crate::models::users::{EditUser, LoginCredentials, NewUser, UpdatedUser, User};
use crate::output::pagination_output::PaginationInfo;
use crate::output::user_output::{LoginResponse, PaginatedUserResponse, UserOutput};
use crate::schema::users::dsl::*;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::auth::{generate_token, hash_password, verify_password};
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use regex::Regex;

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

    fn with_connection<F, R>(&self, f: F) -> R  
    where 
        F: FnOnce(&mut PgConnection) -> R,
        {
            let mut conn = self.get_connection();
            f(&mut conn)
        }

    fn is_valid_email(new_email: &str) -> Result<(), String> {
        let regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if regex.is_match(new_email) {
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }

    fn email_already_exist(&self, new_email: &str) -> Result<(), String> {

        self.with_connection(|conn|{
            match users
            .filter(email.eq(new_email))
            .select(User::as_select())
            .first::<User>(conn)

            {
                Ok(_) => Err("Email already exists".to_string()),
                Err(diesel::result::Error::NotFound) => Ok(()),
                Err(err) => Err(format!("Database error: {}", err)),
            }
        })
    }

    fn get_users(&self, offset: i64, limit: i64) -> Vec<User> {        
        self.with_connection(|conn|{
            users
            .limit(limit)
            .offset(offset)
            .select(User::as_select())
            .load::<User>(conn)
            .expect("Error loading users")
        })
    }

    pub fn get_paginated_users(
        &self,
        offset: i64,
        limit: i64,
        page: u32,
        auth_user: &AuthenticatedUser,
    ) -> Result<PaginatedUserResponse, String> {
        if !auth_user.is_admin {
            return Err("Unauthorized: Only admins can access this resource.".to_string());
        }

        let users_list = self.get_users(offset, limit);
        let modified_results: Vec<UserOutput> =
            users_list.into_iter().map(UserOutput::from_user).collect();

        Ok(PaginatedUserResponse {
            users: modified_results,
            pagination: PaginationInfo {
                current_page: page,
                limit: limit as u32,
                total_items: self.count_users(),
            },
        })
    }

    pub fn get_user(&self, user_id: i32, _auth_user: &AuthenticatedUser) -> Option<UserOutput> {
        self.with_connection(|conn|{
            users
            .find(user_id)
            .select(User::as_select())
            .first::<User>(conn)
            .ok()
            .map(UserOutput::from_user)
        })
    }

    pub fn create_user(&self, mut new_user: NewUser) -> Result<User, String> {
        if new_user.username.is_empty() || new_user.email.is_empty() || new_user.password.is_empty()
        {
            return Err("Username, email, and password are required.".to_string());
        }

        Self::email_already_exist(&self, &new_user.email)?;
        Self::is_valid_email(&new_user.email)?;

        new_user.password = hash_password(&new_user.password);
        
        if new_user.profile_picture_url.is_none() {
            new_user.profile_picture_url = Some("https://yumana.my.id/default-avatar.png".to_string());
        }
        
        self.with_connection(|conn|{
            diesel::insert_into(users)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
            .map_err(|e| format!("Error creating user: {}", e))
        })

    }

    pub fn login(&self, credentials: LoginCredentials) -> Option<LoginResponse> {
        let user = self.authenticate_user(credentials)?;
        let token = generate_token(user.id, user.is_admin);

        Some(LoginResponse {
            token,
            user: UserOutput::from_user(user),
        })
    }

    fn authenticate_user(&self, credentials: LoginCredentials) -> Option<User> {
        let mut conn = self.get_connection();
        let user = users
            .filter(email.eq(credentials.email))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .ok()?;

        if verify_password(&credentials.password, &user.password) {
            Some(user)
        } else {
            None
        }
    }

    // Maybe next time bro
    // pub fn is_admin_user(
    //     &self,
    //     auth_user: &AuthenticatedUser
    // ) -> Result<(), String> {
    //     if !auth_user.is_admin {
    //         return Err("Unauthorized: Only admins can use this feature.".to_string());
    //     }
    //     Ok(())
    // }

    pub fn is_active_user(
        &self,
        auth_user: &AuthenticatedUser
    ) -> Result<(), String> {
        if auth_user.user_id.is_negative() {
            return Err("Unauthorized: Only User can use this feature.".to_string());
        }
        Ok(())
    }

    pub fn edit_user(
        &self,
        user_id: i32,
        mut user: EditUser,
        auth_user: &AuthenticatedUser,
    ) -> Result<User, String> {
        if !auth_user.is_admin {
            return Err("Unauthorized: Only admins can edit users.".to_string());
        }

        let mut conn = self.get_connection();

        if let Some(ref new_password) = user.password {
            user.password = Some(hash_password(new_password));
        }

        if let Some(ref new_email) = user.email {
            Self::is_valid_email(new_email)?;
        }

        diesel::update(users.find(user_id))
            .set(user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error editing user: {}", e))
    }

    pub fn update_user(&self, user_id: i32, mut user: UpdatedUser) -> Result<User, String> {
        let mut conn = self.get_connection();

        if let Some(ref new_password) = user.password {
            user.password = Some(hash_password(new_password));
        }

        if let Some(ref new_email) = user.email {
            Self::is_valid_email(new_email)?;
        }

        diesel::update(users.find(user_id))
            .set(user)
            .returning(User::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error updating user: {}", e))
    }

    pub fn delete_user(&self, user_id: i32, auth_user: &AuthenticatedUser) -> Result<(), String> {
        if !auth_user.is_admin {
            return Err("Unauthorized: Only admins can delete users.".to_string());
        }

        let mut conn = self.get_connection();
        diesel::delete(users.find(user_id))
            .execute(&mut conn)
            .map_err(|e| format!("Error deleting user: {}", e))
            .map(|_| ())
    }

    pub fn count_users(&self) -> i64 {
        let mut conn = self.get_connection();
        users.count().get_result(&mut conn).unwrap_or(0)
    }

    pub fn get_user_name_by_id(&self, user_id: i32) -> Option<String> {
        let mut conn = self.get_connection();

        users
            .filter(id.eq(user_id))
            .select(username)
            .first::<String>(&mut conn)
            .ok()
    }
}
