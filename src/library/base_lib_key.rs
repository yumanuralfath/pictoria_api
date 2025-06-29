use once_cell::sync::Lazy;
use std::env;

pub static OPENROUTER_API_KEY: Lazy<String> = Lazy::new(|| {
    env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY must be set")
});


pub static CLOUDINARY_API_KEY: Lazy<String> = Lazy::new(|| {
    env::var("CLOUDINARY_API_KEY").expect("CLOUDINARY_API_KEY must be set")
});

pub static CLOUDINARY_API_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("CLOUDINARY_API_SECRET").expect("CLOUDINARY_API_SECRET must be set")
});

pub static CLOUDINARY_API_CLOUD_NAME: Lazy<String> = Lazy::new(|| {
    env::var("CLOUDINARY_API_CLOUD_NAME").expect("CLOUDINARY_API_CLOUD_NAME must be set")
});

pub static CLOUDINARY_API_BASE_URL: Lazy<String> = Lazy::new(|| {
    env::var("CLOUDINARY_API_BASE_URL").expect("CLOUDINARY_API_BASE_URL must be set")
}); 