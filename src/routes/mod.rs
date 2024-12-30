pub mod comment_routes;
pub mod routes;
pub mod threads_routes;
pub mod users_routes;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        routes::index,
        users_routes::get_users,
        users_routes::get_user,
        users_routes::create_user,
        users_routes::login_route,
        users_routes::edit_user,
        users_routes::update_user,
        users_routes::me,
        users_routes::delete_user,
        threads_routes::get_threads,
        threads_routes::create_thread,
        threads_routes::update_thread,
        threads_routes::delete_thread,
        comment_routes::create_comment,
        comment_routes::get_comments
    ]
}
