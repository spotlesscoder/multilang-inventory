use crate::infra::db::id;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Collection, Database, IndexModel};
use serde::{Deserialize, Serialize};

mod commands;
mod routes;
pub use routes::*;

pub fn routes() -> axum::Router {
    axum::Router::new().merge(commands::routes())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(default = "id")]
    pub _id: String,
    pub category: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProduct {
    pub category: String,
    pub name: String,
}

impl Product {
    pub fn new(new: CreateProduct) -> Self {
        Self {
            _id: id(),
            category: new.category,
            name: new.name,
        }
    }
}

pub struct ProductsRepository {
    collection: Collection<Product>,
}

impl ProductsRepository {
    pub async fn new(db: &Database) -> anyhow::Result<Self> {
        let collection = db.collection("products");

        // Create index on category
        let options = IndexOptions::builder().build();
        let model = IndexModel::builder()
            .keys(doc! { "category": 1 })
            .options(options)
            .build();
        collection.create_index(model).await?;

        Ok(Self { collection })
    }

    pub async fn insert(&self, product: Product) -> anyhow::Result<()> {
        self.collection.insert_one(product).await?;
        Ok(())
    }

    pub async fn get_by_id(&self, id: &str) -> anyhow::Result<Product> {
        let filter = doc! { "_id": id };
        let product = self.collection.find_one(filter).await?;
        product.ok_or_else(|| anyhow::anyhow!("Product not found"))
    }

    pub async fn get_all_by_category(&self, category: &str) -> anyhow::Result<Vec<Product>> {
        let filter = doc! { "category": category };
        let cursor = self.collection.find(filter).await?;
        let products = cursor.try_collect().await?;
        Ok(products)
    }
}
