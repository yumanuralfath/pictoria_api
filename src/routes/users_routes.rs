use crate::controllers::users_controllers::UserController;
use crate::models::users::{EditUser, LoginCredentials, NewUser, UpdatedUser};
use crate::output::user_output::{
    CreateUserResponse, LoginResponse, PaginatedUserResponse, UserOutput,
};
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use crate::utils::pagination::paginate;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use serde_json::Value;

#[get("/users?<page>&<limit>")]
pub async fn get_users(
    _auth: AuthenticatedUser,
    page: Option<u32>,
    limit: Option<u32>,
    pool: &State<DbPool>,
) -> Result<Json<PaginatedUserResponse>, (Status, Json<Value>)> {
    let controller = UserController::new(pool.inner());
    let (offset, limit_val) = paginate(page, limit);

    match controller.get_users(offset, limit_val, page.unwrap_or(1), &_auth) {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err((Status::Unauthorized, Json(json!({"error": err})))),
    }
}

#[get("/user/<user_id>")]
pub async fn get_user(
    user_id: i32,
    pool: &State<DbPool>,
    _auth: AuthenticatedUser,
) -> Result<Json<UserOutput>, Status> {
    let user_controller = UserController::new(pool.inner());
    match user_controller.get_user_by_id(user_id, &_auth) {
        Some(output) => Ok(Json(output)),
        None => Err(Status::NotFound),
    }
}

#[post("/register", data = "<user>")]
pub async fn create_user(
    user: Json<NewUser>,
    pool: &State<DbPool>,
) -> Result<Json<CreateUserResponse>, (Status, Json<Value>)> {
    let user_controller = UserController::new(pool.inner());

    match user_controller.create_new_user(user.into_inner()) {
        Ok(user) => {
            let response = CreateUserResponse::from_create_user(user);
            Ok(Json(response))
        }
        Err(e) => Err((Status::BadRequest, Json(json!({"error": e})))),
    }
}

#[post("/login", data = "<credentials>")]
pub async fn login_route(
    credentials: Json<LoginCredentials>,
    pool: &State<DbPool>,
) -> Result<Json<LoginResponse>, (Status, Json<Value>)> {
    let controller = UserController::new(pool.inner());

    match controller.login(credentials.into_inner()) {
        Some(response) => Ok(Json(response)),
        None => Err((
            Status::Unauthorized,
            Json(json!({"error": "Invalid credentials"})),
        )),
    }
}

#[put("/user/<user_id>", data = "<user>")]
pub async fn edit_user(
    user_id: i32,
    user: Json<EditUser>,
    pool: &State<DbPool>,
    _auth: AuthenticatedUser,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let user_controller = UserController::new(pool.inner());
    match user_controller.edit_user(user_id, user.into_inner(), &_auth) {
        Ok(updated_user) => Ok(Json(json!({
            "message": "User edited successfully",
            "user": updated_user
        }))),
        Err(e) => Err((
            Status::BadRequest,
            Json(json!({
                "error": e
            })),
        )),
    }
}

#[put("/user", data = "<user>")]
pub async fn update_user(
    auth: AuthenticatedUser,
    user: Json<UpdatedUser>,
    pool: &State<DbPool>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let user_controller = UserController::new(pool.inner());
    match user_controller.update_user(auth.user_id, user.into_inner()) {
        Ok(updated_user) => Ok(Json(json!({
            "message": "User Update successfully",
            "user": updated_user
        }))),
        Err(e) => Err((
            Status::BadRequest,
            Json(json!({
                "error": e
            })),
        )),
    }
}

#[delete("/user/<user_id>")]
pub async fn delete_user(
    user_id: i32,
    pool: &State<DbPool>,
    _auth: AuthenticatedUser,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let user_controller = UserController::new(pool.inner());
    match user_controller.delete_user(user_id, &_auth) {
        Ok(_) => Ok(Json(json!({
            "message": "User deleted successfully",
        }))),
        Err(e) => Err((
            Status::BadRequest,
            Json(json!({
                "error": e
            })),
        )),
    }
}

#[get("/me")]
pub async fn me(auth: AuthenticatedUser, pool: &State<DbPool>) -> Result<Json<Value>, Status> {
    let user_controller = UserController::new(pool.inner());
    let user = user_controller.get_user_by_id(auth.user_id, &auth);
    Ok(Json(json!({
        "user": user
    })))
}
