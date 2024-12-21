#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "welcome to the world of pictoria"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}