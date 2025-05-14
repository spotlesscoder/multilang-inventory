use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::time::Duration;

use super::{CreateProduct, Product, ProductsRepository};
use crate::infra::context::AppContext;

pub async fn get_product_by_id(
    Path(id): Path<String>,
    Extension(context): Extension<AppContext>,
) -> Result<impl IntoResponse, StatusCode> {
    // Try to get from cache first
    let cache_key = format!("product:{}", id);
    if let Some(cached_product) = context.cache.get::<Product>(&cache_key).await.unwrap() {
        return Ok(Json(json!({
            "source": "cache",
            "product": cached_product
        })));
    }

    // If not in cache, get from database
    let products_repo = ProductsRepository::new(&context.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let product = products_repo
        .get_by_id(&id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    // Store in cache with 1 hour TTL
    context
        .cache
        .set_with_ttl(&cache_key, &product, Duration::from_secs(3600))
        .await
        .unwrap();

    Ok(Json(json!({
        "source": "database",
        "product": product
    })))
}
