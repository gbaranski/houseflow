use super::TokenStore;
use async_trait::async_trait;
use houseflow_token::{Token, TokenID};
use redis_client::{aio::Connection, Client, AsyncCommands};
use std::sync::Arc;
use tokio::sync::Mutex;

pub use redis_client::RedisError as Error;

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
impl TokenStore<Error> for RedisTokenStore {
    async fn exists(&self, id: &TokenID) -> Result<bool, super::Error<Error>> {
        Ok(self
            .connection
            .lock()
            .await
            .exists::<_, bool>(id.to_string())
            .await?)
    }

    async fn set(&self, token: &Token) -> Result<(), super::Error<Error>> {
        self.connection
            .lock()
            .await
            .set(token.payload.id.to_string(), token.base64())
            .await?;

        Ok(())
    }
}
