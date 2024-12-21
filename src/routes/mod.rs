pub mod routes;
pub mod users;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        routes::index,
        users::get_users,
        users::get_user,
        users::create_user,
    ]
}
