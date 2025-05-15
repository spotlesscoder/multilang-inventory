mod infra;
mod modules;
mod tests;

use axum::{http::Method, Extension, Router};
use infra::{auth::AuthUser, context::AppContext, db::db};
use std::{sync::Arc, time::Duration};
use tower_governor::{governor::GovernorConfigBuilder, key_extractor::KeyExtractor, GovernorLayer};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let db = db("test_db").await.unwrap();
    let cache = crate::infra::cache::RedisCache::new().unwrap();
    let ctx = AppContext::new(db, cache);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    let api_routes = Router::new()
        .merge(infra::routes())
        .merge(modules::routes());

    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(20)
            .burst_size(5)
            .use_headers()
            .finish()
            .unwrap(),
    );

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(Extension(ctx))
        .layer(cors)
        .layer(GovernorLayer {
            config: governor_conf,
        });

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listen_addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&listen_addr).await.unwrap();

    info!("🚀 Server listening on {}", listen_addr);
    axum::serve(listener, app).await.unwrap();
}
