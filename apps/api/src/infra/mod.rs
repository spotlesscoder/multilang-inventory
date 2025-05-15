use axum::routing::get;
use health::health_check;

pub(crate) mod auth;
pub(crate) mod cache;
pub(crate) mod context;
pub(crate) mod db;
pub(crate) mod health;
pub(crate) mod rate_limiter;

pub fn routes() -> axum::Router {
    axum::Router::new().route("/health", get(health_check))
}
