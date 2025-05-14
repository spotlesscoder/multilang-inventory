use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};

use crate::infra::auth::{create_token, Role};

#[derive(Debug, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

pub async fn login_handler(
    Json(credentials): Json<Credentials>,
) -> Result<impl IntoResponse, StatusCode> {
    // In a real application, you would verify against a database
    // This is just an example implementation
    if credentials.username == "admin" {
        let hashed = hash("admin123", DEFAULT_COST).unwrap();
        if verify(&credentials.password, &hashed).unwrap() {
            let token = create_token(&credentials.username, Role::Admin)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            return Ok((StatusCode::OK, Json(AuthResponse { token })));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
