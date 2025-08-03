use crate::output::root_output::ApiInfo;
use crate::utils::db::DbPool;
use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_query;
use rocket::State;
use rocket::{get, serde::json::Json};
use std::collections::HashMap;

#[get("/")]
pub fn index(db_pool: &State<DbPool>) -> Json<ApiInfo<'static>> {
    let mut links = HashMap::new();
    links.insert("users", "/users");
    links.insert("threads", "/threads");
    links.insert("chats", "/chats");
    links.insert("log_books", "/log_books");

    let db_status = match db_pool.get() {
        Ok(mut conn) => match sql_query("SELECT 1").execute(&mut conn) {
            Ok(_) => "healthy",
            Err(_) => "unhealthy",
        },
        Err(_) => "unhealthy",
    };

    Json(ApiInfo {
        name: "Yumana API",
        version: env!("CARGO_PKG_VERSION"),
        description: "API for backend yuma nur alfath website.",
        status: db_status,
        timestamp: Utc::now().to_rfc3339(),
        documentation: "/api/docs",
        links,
    })
}
