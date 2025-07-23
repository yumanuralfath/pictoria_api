use once_cell::sync::Lazy;
use std::env;

macro_rules! lazy_env {
    ($key:ident) => {
        pub static $key: Lazy<String> = Lazy::new(|| {
            env::var(stringify!($key)).expect(concat!(stringify!($key), " must be set"))
        });
    };
}

lazy_env!(OPENROUTER_API_KEY);
lazy_env!(CLOUDINARY_API_KEY);
lazy_env!(CLOUDINARY_API_SECRET);
lazy_env!(CLOUDINARY_API_CLOUD_NAME);
lazy_env!(CLOUDINARY_API_BASE_URL);
lazy_env!(REDIS_URL);
lazy_env!(WEBHOOK_IP_BAN_URL);
lazy_env!(WEBHOOK_IP_BAN_USER);
lazy_env!(WEBHOOK_IP_BAN_PASSWORD);