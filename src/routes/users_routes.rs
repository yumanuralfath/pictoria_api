use crate::controllers::users_controllers::UserController;
use crate::models::users::{NewUser, User};
use crate::utils::db::DbPool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use serde_json::Value;

#[get("/users")]
pub async fn get_users(pool: &State<DbPool>) -> Json<Vec<User>> {
    let user_controller = UserController::new(pool.inner());
    let results = user_controller.get_all_users();
    Json(results)
}

#[get("/user/<user_id>")]
pub async fn get_user(user_id: i32, pool: &State<DbPool>) -> Result<Json<User>, Status> {
    let user_controller = UserController::new(pool.inner());
    match user_controller.get_user_by_id(user_id) {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound),
    }
}

#[post("/users", data = "<user>")]
pub async fn create_user(
    user: Json<NewUser>,
    pool: &State<DbPool>,
) -> Result<Json<User>, (Status, Json<Value>)> {
    let user_controller = UserController::new(pool.inner());

    if user.username.is_empty() || user.email.is_empty() || user.password.is_empty() {
        return Err((
            Status::BadRequest,
            Json(json!({"error": "Username, email, and password are required."})),
        ));
    }

    let new_user = user_controller.create_new_user(user.into_inner());
    Ok(Json(new_user))
}
