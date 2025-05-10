use db::db;
mod db;
mod products;
mod tests;

#[tokio::main]
async fn main() {
    let db = db("test_db").await.unwrap();
    let products_repo = products::ProductsRepository::new(&db).await.unwrap();
    let product = products::Product::new(products::CreateProduct {
        category: "electronics".to_string(),
        name: "product".to_string(),
    });
    products_repo.insert(product.clone()).await.unwrap();

    let all_products = products_repo
        .get_all_by_category("electronics")
        .await
        .unwrap();
    for product in all_products {
        println!("Product: {:?}", product);
    }
}
