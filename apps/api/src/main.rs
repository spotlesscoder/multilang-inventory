mod infra;
mod products;
mod tests;

use axum::{routing::get, Router};
use infra::{
    auth::{authorize, AuthUser, Role},
    db::db,
};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let db = db("test_db").await.unwrap();
    let cache = crate::infra::cache::RedisCache::new().unwrap();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/protected", get(protected_route))
        .route("/api/admin", get(admin_route))
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listen_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&listen_addr).await.unwrap();
    tracing::info!("Server listening on {}", listen_addr);
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn protected_route(user: AuthUser) -> Result<String, axum::http::StatusCode> {
    authorize(&user, Role::User).await?;
    Ok(format!("Protected content for user: {}", user.user_id))
}

async fn admin_route(user: AuthUser) -> Result<String, axum::http::StatusCode> {
    authorize(&user, Role::Admin).await?;
    Ok("Admin only content".to_string())
}
