use anyhow::Result;
use redis::{aio::Connection, AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use std::{env, time::Duration};

#[derive(Clone)]
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub fn new() -> Result<Self> {
        let uri = env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let client = redis::Client::open(uri)?;
        Ok(Self { client })
    }

    async fn get_connection(&self) -> Result<Connection> {
        Ok(self.client.get_async_connection().await?)
    }

    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        expiry_ms: Option<u128>,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let serialized = serde_json::to_string(value)?;

        match expiry_ms {
            Some(secs) => {
                conn.pset_ex(key, serialized, secs as usize).await?;
            }
            None => {
                conn.set(key, serialized).await?;
            }
        }

        Ok(())
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let mut conn = self.get_connection().await?;
        let value: Option<String> = conn.get(key).await?;

        match value {
            Some(val) => Ok(Some(serde_json::from_str(&val)?)),
            None => Ok(None),
        }
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        conn.del(key).await?;
        Ok(())
    }

    pub async fn clear_all(&self) -> Result<()> {
        let mut conn = self.get_connection().await?;
        redis::cmd("FLUSHALL").query_async(&mut conn).await?;
        Ok(())
    }

    pub async fn set_with_ttl<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<()> {
        self.set(key, value, Some(ttl.as_millis())).await
    }
}
