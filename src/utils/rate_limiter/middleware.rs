//change to global rate limiter instead for specific endpoint

use std::{env, net::IpAddr};
use redis::Commands;
use rocket::{
    http::Status, request::{FromRequest, Outcome}, tokio, Request
};

use crate::utils::rate_limiter::ip_ban::send_block_webhook;

pub struct RateLimiter;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RateLimiter {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // skip for development
        if env::var("ENV_ENVIRONMENT").unwrap_or_else(|_| "DEVELOPMENT".into()) == "DEVELOPMENT" {
            return Outcome::Success(RateLimiter);
        }


        let client_ip: IpAddr = match req.client_ip() {
            Some(ip) => ip,
            None => return Outcome::Forward(Status::BadRequest),
        };

        let redis_client = match req.rocket().state::<redis::Client>() {
            Some(client) => client,
            None => return Outcome::Forward(Status::InternalServerError),
        };

        let mut con = match redis_client.get_connection() {
            Ok(c) => c,
            Err(_) => return Outcome::Forward(Status::InternalServerError),
        };

        let key = format!("rate_limit:{}", client_ip);
        let count: i32 = con.incr(&key, 1).unwrap_or(0);
        let _: () = con.expire(&key, 60).unwrap_or(());

        if count > 100 {
            println!("ABUSE DETECTED: {}", client_ip);
            tokio::spawn(async move {
                send_block_webhook(client_ip, None).await;
            });
            return Outcome::Forward(Status::TooManyRequests);
        }

        Outcome::Success(RateLimiter)
    }
}
