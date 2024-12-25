use crate::models::users::{LoginCredentials, NewUser, User};
use crate::schema::users::dsl::*;
use crate::utils::auth::{hash_password, verify_password};
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

    pub(crate) fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get DB connection")
    }

    pub(crate) fn get_users(&self, offset: i64, limit: i64) -> Vec<User> {
        let mut conn = self.get_connection();
        users
            .limit(limit)
            .offset(offset)
            .load::<User>(&mut conn)
            .expect("Error loading users")
    }

    pub(crate) fn get_user(&self, user_id: i32) -> Option<User> {
        let mut conn = self.get_connection();
        users.find(user_id).first(&mut conn).ok()
    }

    pub fn create_user(&self, mut new_user: NewUser) -> User {
        let mut conn = self.get_connection();
        new_user.password = hash_password(&new_user.password);

        diesel::insert_into(users)
            .values(new_user)
            .get_result(&mut conn)
            .expect("Error creating user")
    }

    pub fn login_user(&self, credentials: LoginCredentials) -> Option<User> {
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

    pub fn count_users(&self) -> i64 {
        let mut conn = self.get_connection();
        users.count().get_result(&mut conn).unwrap_or(0)
    }
}
