use mongodb::{Client, Database};
use std::env;

pub(crate) fn id() -> String {
    nanoid::nanoid!()
}

pub(crate) async fn db(db_name: &str) -> mongodb::error::Result<Database> {
    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://mongo:27017".to_string());
    let client = Client::with_uri_str(&uri).await?;
    Ok(client.database(db_name))
}
