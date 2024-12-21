use crate::controllers::user_controller::UserController;
use crate::models::users::{NewUser, User};
use crate::utils::db::DbPool;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[get("/users")]
pub async fn get_users(pool: &State<DbPool>) -> Json<Vec<User>> {
    let user_controller = UserController::new(pool.inner());
    let results = user_controller.get_all_users();
    Json(results)
}

#[get("/user/<user_id>")]
pub async fn get_user(user_id: i32, pool: &State<DbPool>) -> Json<User> {
    let user_controller = UserController::new(pool.inner());
    let user = user_controller.get_user_by_id(user_id);
    Json(user)
}

#[post("/users", data = "<user>")]
pub async fn create_user(user: Json<NewUser>, pool: &State<DbPool>) -> (Status, Json<User>) {
    let user_controller = UserController::new(pool.inner());
    let new_user = user_controller.create_new_user(user.into_inner());
    (Status::Created, Json(new_user))
}
