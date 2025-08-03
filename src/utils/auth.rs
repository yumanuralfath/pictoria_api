use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub is_admin: bool,
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub is_admin: bool,
}

//wkwkkw pindah kesini
impl AuthenticatedUser {
    pub fn is_admin_user(&self) -> Result<(), String> {
        if !self.is_admin {
            return Err("Unauthorized: Only admins can use this feature.".to_string());
        }
        Ok(())
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization");

        match token {
            Some(token) if token.starts_with("Bearer ") => {
                let token = token.replace("Bearer ", "");
                let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                let decoding_key = DecodingKey::from_secret(secret.as_bytes());
                let validation = Validation::default();

                match decode::<Claims>(&token, &decoding_key, &validation) {
                    Ok(token_data) => Outcome::Success(AuthenticatedUser {
                        user_id: token_data.claims.sub,
                        is_admin: token_data.claims.is_admin,
                    }),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

pub fn hash_password(password: &str) -> String {
    hash(password.as_bytes(), DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password.as_bytes(), hash).unwrap_or(false)
}

pub fn generate_token(user_id: i32, is_admin: bool) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        is_admin,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
