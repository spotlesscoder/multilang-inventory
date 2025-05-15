mod infra;
mod products;
mod tests;

use axum::{
    http::Method,
    routing::{get, post},
    Extension, Router,
};
use infra::{
    auth::{authorize, AuthUser, Role},
    context::AppContext,
    db::db,
};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let db = db("test_db").await.unwrap();
    let cache = crate::infra::cache::RedisCache::new().unwrap();

    // Initialize AppContext
    let ctx = AppContext::new(db, cache);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    // API routes
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/protected", get(protected_route))
        .route("/admin", get(admin_route))
        // Product routes
        .route("/products/:id", get(products::get_product_by_id));

    // Main app with middleware and mounted routes
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(Extension(ctx))
        .layer(cors);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listen_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&listen_addr).await.unwrap();

    info!("🚀 Server listening on {}", listen_addr);
    axum::serve(listener, app).await.unwrap();
}

// Update route handlers to use AppContext
async fn health_check() -> &'static str {
    "OK"
}

async fn protected_route(
    Extension(context): Extension<AppContext>,
    user: AuthUser,
) -> Result<String, axum::http::StatusCode> {
    authorize(&user, Role::User).await?;
    Ok(format!("Protected content for user: {}", user.user_id))
}

async fn admin_route(
    Extension(context): Extension<AppContext>,
    user: AuthUser,
) -> Result<String, axum::http::StatusCode> {
    authorize(&user, Role::Admin).await?;
    Ok("Admin only content".to_string())
}
