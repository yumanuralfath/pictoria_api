use rocket::{Request, Data, fairing::{Fairing, Info, Kind}, http::Status, Response};
use std::env;
use redis::Commands;
use rocket::tokio;
use crate::utils::rate_limiter::ip_ban::send_block_webhook;
use crate::utils::rate_limiter::get_real_ip::get_real_ip;

pub struct GlobalRateLimiter;

#[rocket::async_trait]
impl Fairing for GlobalRateLimiter {
    fn info(&self) -> Info {
        Info {
            name: "Global IP Rate Limiter",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        // Skip if development
        if env::var("ENV_ENVIRONMENT").unwrap_or_else(|_| "DEVELOPMENT".into()) == "DEVELOPMENT" {
            return;
        }

        let client_ip = match get_real_ip(request) {
            Some(ip) => ip,
            None => return,
        };

        if let Some(redis_client) = request.rocket().state::<redis::Client>() {
            if let Ok(mut con) = redis_client.get_connection() {
                let key = format!("rate_limit:{}", client_ip);
                let count: i32 = con.incr(&key, 1).unwrap_or(0);
                let _: () = con.expire(&key, 60).unwrap_or(());

                if count > 100 {
                    println!("ABUSE DETECTED: {}", client_ip);
                    tokio::spawn(async move {
                        send_block_webhook(client_ip, None).await;
                    });

                    request.local_cache(|| true);
                }
            }
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if *request.local_cache::<bool, _>(|| false) {
            response.set_status(Status::TooManyRequests);
            response.set_sized_body("Rate limit exceeded".len(), std::io::Cursor::new("Rate limit exceeded"));
        }
    }
}