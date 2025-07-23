use rocket::get;
use crate::utils::rate_limiter::middleware::RateLimiter;

#[get("/")]
pub fn index(_rate_limiter: RateLimiter) -> &'static str {
    "Welcome to Yumana API!"
}
