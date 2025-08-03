// tests/common/mod.rs

// use chrono::Local;
use diesel::prelude::*;
use rocket::local::blocking::Client;
// use yumana_api::models::log_books::{LogBook, NewLogBook};
use yumana_api::models::users::{NewUser, User};
use yumana_api::schema::{log_books, users};
use yumana_api::utils::auth::hash_password;
use yumana_api::utils::db::DbPool;

// --- Test User Guard ---

/// Manages the lifecycle of a test user.
/// Creates a user on instantiation and deletes it when it goes out of scope.
/// This ensures cleanup even if a test panics.
pub struct TestUserGuard {
    pub user_id: i32,
    pool: DbPool,
}

pub struct TestLoogBookGuard {
    pub log_book_id: i32,
    pool: DbPool,
}

impl TestUserGuard {
    /// Creates a new user in the database with default credentials.
    pub fn new(pool: &DbPool) -> Self {
        let mut conn = pool
            .get()
            .expect("Failed to get DB connection for test setup");

        let new_user = NewUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: hash_password("password123"),
            profile_picture_url: None,
        };

        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result::<User>(&mut conn)
            .expect("Failed to insert test user");

        TestUserGuard {
            user_id: user.id,
            pool: pool.clone(),
        }
    }
}

// impl TestLoogBookGuard {
//     pub fn new(pool: &DbPool, test_user_id: i32, content: &str) -> Self {
//         let mut conn = pool
//             .get()
//             .expect("Gagagl mendapatkan koneksi DB untuk setup local");

//         let new_log = NewLogBook {
//             user_id: test_user_id,
//             content: content.to_string(),
//             date: Local::now().naive_local().date(),
//         };

//         let log_book = diesel::insert_into(log_books::table)
//             .values(&new_log)
//             .returning(LogBook::as_returning())
//             .get_result::<LogBook>(&mut conn)
//             .expect("Gagal mebuat log book test");

//         TestLoogBookGuard {
//             log_book_id: log_book.id,
//             pool: pool.clone(),
//         }
//     }
// }

impl Drop for TestUserGuard {
    fn drop(&mut self) {
        let mut conn = self
            .pool
            .get()
            .expect("Failed to get DB connection for test cleanup");
        diesel::delete(users::table.find(self.user_id))
            .execute(&mut conn)
            .expect("Failed to delete test user");
    }
}

impl Drop for TestLoogBookGuard {
    fn drop(&mut self) {
        let mut conn = self
            .pool
            .get()
            .expect("Gagal mendapatkan koneksi DB untuk clean");

        diesel::delete(log_books::table.find(self.log_book_id))
            .execute(&mut conn)
            .expect("Gagal menghapus log book tes");
    }
}

// --- Test Setup Helpers ---

/// Sets up the Rocket client for testing.
pub fn setup_client() -> Client {
    let rocket_instance = yumana_api::rocket();
    Client::tracked(rocket_instance).expect("Failed to create a valid Rocket instance for testing")
}

/// A convenience function that sets up a client and creates a test user.
/// Returns the client and the user guard, which ensures the user is cleaned up.
pub fn setup_with_user() -> (Client, TestUserGuard) {
    let client = setup_client();
    let pool = client
        .rocket()
        .state::<DbPool>()
        .expect("Failed to get DB pool");
    let user_guard = TestUserGuard::new(pool);
    (client, user_guard)
}
