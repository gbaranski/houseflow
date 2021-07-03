use super::{TokenStore, TokenStoreInternalError};
use async_trait::async_trait;
use houseflow_types::token::RefreshTokenID;
use redis::{aio::Connection, AsyncCommands, Client};
use std::sync::Arc;
use tokio::sync::Mutex;

pub use redis::RedisError as Error;

impl TokenStoreInternalError for Error {}

#[derive(Clone)]
pub struct RedisTokenStore {
    connection: Arc<Mutex<Connection>>,
}

impl RedisTokenStore {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::open("redis://127.0.0.1")?;
        let connection = client.get_tokio_connection().await?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
}

#[async_trait]
impl TokenStore for RedisTokenStore {
    async fn exists(&self, id: &RefreshTokenID) -> Result<bool, super::Error> {
        Ok(self
            .connection
            .lock()
            .await
            .exists::<_, bool>(id.to_string())
            .await?)
    }

    async fn remove(&self, id: &RefreshTokenID) -> Result<bool, super::Error> {
        let removed: bool = self.connection.lock().await.del(id.to_string()).await?;
        Ok(removed)
    }

    async fn add(&self, id: &RefreshTokenID) -> Result<(), super::Error> {
        self.connection
            .lock()
            .await
            .set(id.to_string(), "")
            .await?;

        Ok(())
    }
}
