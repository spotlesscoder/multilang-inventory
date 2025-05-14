pub fn route() -> axum::Router {
    axum::Router::new()
        .route("/products", axum::routing::delete(delete_handler))
        .layer(axum::AddExtensionLayer::new(AppContext::new(
            db("products").await.unwrap(),
            RedisCache::new().await.unwrap(),
        )))
}

