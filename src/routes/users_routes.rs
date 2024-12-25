use crate::controllers::users_controllers::UserController;
use crate::models::users::{LoginCredentials, NewUser};
use crate::output::user_output::{
    CreateUserResponse, PaginatedUserResponse, PaginationInfo, UserOutput,
};
use crate::utils::auth::{generate_token, AuthenticatedUser};
use crate::utils::db::DbPool;
use crate::utils::pagination::paginate;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use serde_json::Value;

fn get_user_controller(pool: &State<DbPool>) -> UserController {
    UserController::new(pool.inner())
}

#[get("/users?<page>&<limit>")]
pub async fn get_users(
    _auth: AuthenticatedUser,
    page: Option<u32>,
    limit: Option<u32>,
    pool: &State<DbPool>,
) -> Json<PaginatedUserResponse> {
    let user_controller = get_user_controller(pool);
    let (offset, limit_val) = paginate(page, limit);

    let results = user_controller.get_all_users(offset, limit_val);
    let modified_results: Vec<UserOutput> =
        results.into_iter().map(UserOutput::from_user).collect();

    let response = PaginatedUserResponse {
        users: modified_results,
        pagination: PaginationInfo {
            current_page: page.unwrap_or(1),
            limit: limit_val as u32,
            total_items: user_controller.get_total_users(),
        },
    };

    Json(response)
}

#[get("/user/<user_id>")]
pub async fn get_user(
    user_id: i32,
    pool: &State<DbPool>,
    _auth: AuthenticatedUser,
) -> Result<Json<UserOutput>, Status> {
    let user_controller = get_user_controller(pool);
    match user_controller.get_user_by_id(user_id) {
        Some(user) => {
            let output = UserOutput::from_user(user);
            Ok(Json(output))
        }
        None => Err(Status::NotFound),
    }
}

#[post("/register", data = "<user>")]
pub async fn create_user(
    user: Json<NewUser>,
    pool: &State<DbPool>,
) -> Result<Json<CreateUserResponse>, (Status, Json<Value>)> {
    let user_controller = get_user_controller(pool);

    if user.username.is_empty() || user.email.is_empty() || user.password.is_empty() {
        return Err((
            Status::BadRequest,
            Json(json!({"error": "Username, email, and password are required."})),
        ));
    }

    let result = user_controller.create_new_user(user.into_inner());
    let response = CreateUserResponse::from_create_user(result);
    Ok(Json(response))
}

#[post("/login", data = "<credentials>")]
pub async fn login_route(
    credentials: Json<LoginCredentials>,
    pool: &State<DbPool>,
) -> Result<Json<Value>, (Status, Json<Value>)> {
    let user_controller = get_user_controller(pool);

    match user_controller.login_controller(credentials.into_inner()) {
        Some(user) => {
            let token = generate_token(user.id);
            Ok(Json(json!({
                "token": token,
                "user": {
                    "id": user.id,
                    "email": user.email,
                    "username": user.username
                }
            })))
        }
        None => Err((
            Status::Unauthorized,
            Json(json!({"error": "Invalid credentials"})),
        )),
    }
}
