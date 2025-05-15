use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{
    infra::context::AppContext,
    products::{CreateProduct, Product, ProductsRepository},
};

pub async fn exec(
    Extension(context): Extension<AppContext>,
    Json(create_product): Json<CreateProduct>,
) -> Result<impl IntoResponse, StatusCode> {
    let products_repo = ProductsRepository::new(&context.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product = Product::new(create_product);
    products_repo
        .insert(product.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Store in cache
    let cache_key = format!("product:{}", product._id);
    context
        .cache
        .set_with_ttl(&cache_key, &product, Duration::from_secs(3600))
        .await
        .unwrap();

    Ok((StatusCode::CREATED, Json(product)))
}
