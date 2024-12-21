use crate::models::users::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::utils::db::DbPool;
use diesel::prelude::*;

pub struct UserService<'a> {
    pool: &'a DbPool,
}

impl<'a> UserService<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        UserService { pool }
    }

    pub(crate) fn get_users(&self) -> Vec<User> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        users.load::<User>(&mut conn).expect("Error loading users")
    }

    pub(crate) fn get_user(&self, user_id: i32) -> User {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        users
            .find(user_id)
            .first(&mut conn)
            .expect("Error loading user")
    }

    pub(crate) fn create_user(&self, new_user: NewUser) -> User {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(users)
            .values(new_user)
            .get_result(&mut conn)
            .expect("Error creating user")
    }
}
