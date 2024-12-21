use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use dotenvy::dotenv;
use rocket::fairing::AdHoc;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db_pool() -> DbPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn attach_db() -> AdHoc {
    AdHoc::on_ignite("Database Pool", |rocket| async {
        let pool = init_db_pool();
        rocket.manage(pool)
    })
}
