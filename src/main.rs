#[macro_use]
extern crate rocket;

mod controllers;
mod models;
mod routes;
mod schema;
mod services;
mod utils;

use crate::routes::get_routes;
use dotenvy::dotenv;
use std::env;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let port: u16 = port.parse().expect("PORT harus berupa angka");
    let address = env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());

    rocket::custom(
        rocket::Config::figment()
            .merge(("port", port))
            .merge(("address", address)),
    )
    .mount("/", get_routes())
    .attach(utils::db::attach_db())
}
