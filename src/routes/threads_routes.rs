use crate::controllers::threads_controllers::ThreadController;
use crate::output::thread_output::PaginatedThreadResponse;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use crate::utils::pagination::paginate;
use rocket::serde::json::Json;
use rocket::State;

#[get("/threads?<page>&<limit>")]
pub async fn get_threads(
    _auth: AuthenticatedUser,
    page: Option<u32>,
    limit: Option<u32>,
    pool: &State<DbPool>,
) -> Json<PaginatedThreadResponse> {
    let controller = ThreadController::new(pool.inner());
    let (offset, limit_val) = paginate(page, limit);

    Json(controller.get_paginated_threads(offset, limit_val, page.unwrap_or(1), &_auth))
}
