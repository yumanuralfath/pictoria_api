pub mod chats_routes;
pub mod cloudinary_routes;
pub mod comment_routes;
pub mod log_books_routes;
pub mod openai_routes;
pub mod routes;
pub mod threads_routes;
pub mod users_routes;
pub mod voices_routes;

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
        users_routes::get_username_by_id,
        users_routes::ip_ban,
        threads_routes::get_threads,
        threads_routes::create_thread,
        threads_routes::update_thread,
        threads_routes::delete_thread,
        comment_routes::create_comment,
        comment_routes::get_comments,
        comment_routes::get_number_comments_by_thread,
        comment_routes::update_comment,
        comment_routes::delete_comment,
        chats_routes::create_chat,
        voices_routes::save_voice,
        voices_routes::update_voice,
        voices_routes::delete_voice,
        openai_routes::generate,
        voices_routes::get_voice_log_by_date,
        voices_routes::weekly_resume,
        voices_routes::monthly_resume,
        voices_routes::get_active_voice_month,
        cloudinary_routes::upload_profile_pic,
        log_books_routes::create_log_book,
        log_books_routes::get_log_books,
        log_books_routes::get_log_book_by_id,
        log_books_routes::update_log_book,
        log_books_routes::delete_log_book
    ]
}
