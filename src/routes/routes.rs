use rocket::get;

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to Pictoria API!"
}