pub mod add_controller;
pub mod add_handler;

pub fn routes() -> axum::Router {
    axum::Router::new().merge(add_controller::route())
}
