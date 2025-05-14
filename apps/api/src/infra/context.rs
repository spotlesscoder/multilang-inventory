use crate::infra::cache::RedisCache;
use mongodb::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContext {
    pub db: Database,
    pub cache: Arc<RedisCache>,
}

impl AppContext {
    pub fn new(db: Database, cache: RedisCache) -> Self {
        Self {
            db,
            cache: Arc::new(cache),
        }
    }
}
