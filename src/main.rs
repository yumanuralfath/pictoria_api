#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod output;
mod routes;
mod schema;
mod services;
mod utils;
mod library;

use crate::routes::get_routes;
use dotenvy::dotenv;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::env;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port: u16 = port.parse().expect("PORT harus berupa angka");
    let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());

    let environment = env::var("ENV_ENVIRONMENT").unwrap_or_else(|_| "PRODUCTION".to_string());

    let allowed_origins = if environment == "DEVELOPMENT" {
        AllowedOrigins::all()
    } else {
        AllowedOrigins::some_exact(&["http://localhost:3000", "https://www.yumana.my.id"])
    };

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error konfigurasi CORS");

    rocket::custom(
        rocket::Config::figment()
            .merge(("port", port))
            .merge(("address", address)),
    )
    .mount("/", get_routes())
    .attach(utils::db::attach_db())
    .attach(cors)
}
