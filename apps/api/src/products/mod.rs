use crate::db::id;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Collection, Database, IndexModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Product {
    #[serde(default = "id")]
    pub _id: String,
    pub category: String,
    pub name: String,
}

pub(crate) struct CreateProduct {
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

pub(crate) struct ProductsRepository {
    collection: Collection<Product>,
}

impl ProductsRepository {
    pub async fn new(db: &Database) -> mongodb::error::Result<Self> {
        let collection = db.collection::<Product>("products");

        // Add index creation here
        let index = IndexModel::builder()
            .keys(doc! { "category": 1 })
            .options(
                IndexOptions::builder()
                    .name(Some("category".into()))
                    .build(),
            )
            .build();

        collection.create_index(index).await?;

        Ok(Self { collection })
    }

    pub async fn insert(&self, product: Product) -> mongodb::error::Result<()> {
        self.collection.insert_one(product).await.map(|_| ())
    }

    pub async fn get_all_by_category(
        &self,
        category: &str,
    ) -> mongodb::error::Result<Vec<Product>> {
        self.collection
            .find(doc! { "category": category })
            .await?
            .try_collect::<Vec<_>>()
            .await
    }

    pub async fn get_by_id(&self, id: &str) -> mongodb::error::Result<Option<Product>> {
        self.collection.find_one(doc! { "_id": id }).await
    }
}

mod test {
    use crate::{
        products::{CreateProduct, Product, ProductsRepository},
        tests::test_db,
    };

    #[tokio::test]
    async fn test_insert_and_retrieve_product() -> Result<(), anyhow::Error> {
        // setup
        let db = test_db().await?;
        let repo = ProductsRepository::new(&db).await?;

        let product = Product::new(CreateProduct {
            category: "electronics".to_string(),
            name: "product".to_string(),
        });

        // run + verify
        repo.insert(product.clone()).await?;
        let products_by_category = repo.get_all_by_category("electronics").await?;

        assert_eq!(products_by_category.len(), 1);
        assert_eq!(products_by_category[0].name, product.name);

        let product_by_id = repo.get_by_id(&product._id).await?;
        assert_eq!(product_by_id.unwrap().name, product.name);

        db.drop().await?;
        Ok(())
    }
}
