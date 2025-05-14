use crate::infra::{context::AppContext, db};

use super::add_handler;

pub fn route() -> axum::Router {
    axum::Router::new().route("/products", axum::routing::delete(add_handler::exec))
}
