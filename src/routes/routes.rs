use rocket::get;

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to Yumana API!"
}
