// tests/login_tests.rs

// Declare the common module, allowing access to its public functions and structs.
mod common;

use crate::common::{setup_client, setup_with_user};
use rocket::http::{ContentType, Status};

#[test]
fn test_successful_login() {
    // Use the combined setup function for conciseness.
    // `_user_guard` is kept to ensure its Drop logic runs at the end of the test.
    let (client, _user_guard) = setup_with_user();

    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .body(r#"{"email": "test@example.com", "password": "password123"}"#)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().unwrap();
    let json_body: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert!(json_body["token"].is_string());
    assert_eq!(json_body["user"]["email"], "test@example.com");
}

#[test]
fn test_failed_login_wrong_password() {
    // The setup function creates and manages the user for this test.
    let (client, _user_guard) = setup_with_user();

    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .body(r#"{"email": "test@example.com", "password": "wrongpassword"}"#)
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    let body = response.into_string().unwrap();
    assert!(body.contains("Invalid credentials"));
}

#[test]
fn test_failed_login_user_not_found() {
    // For this test, we only need the client, not a pre-existing user.
    let client = setup_client();

    let response = client
        .post("/login")
        .header(ContentType::JSON)
        .body(r#"{"email": "nouser@example.com", "password": "password123"}"#)
        .dispatch();

    assert_eq!(response.status(), Status::Unauthorized);
    let body = response.into_string().unwrap();
    assert!(body.contains("Invalid credentials"));
}
