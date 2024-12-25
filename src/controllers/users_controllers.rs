use crate::models::users::{NewUser, User};
use crate::services::users_services::UserService;
use crate::utils::db::DbPool;

pub struct UserController<'a> {
    user_service: UserService<'a>,
}

impl<'a> UserController<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        UserController {
            user_service: UserService::new(pool),
        }
    }

    pub fn get_all_users(&self, offset: i64, limit: i64) -> Vec<User> {
        self.user_service.get_users(offset, limit)
    }

    pub fn get_user_by_id(&self, user_id: i32) -> Option<User> {
        self.user_service.get_user(user_id)
    }

    pub fn create_new_user(&self, new_user: NewUser) -> User {
        self.user_service.create_user(new_user)
    }

    pub fn get_total_users(&self) -> i64 {
        self.user_service.count_users()
    }
}
