pub mod products;
pub mod users;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .merge(products::routes())
        .merge(users::routes())
}
