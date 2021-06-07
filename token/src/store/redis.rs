use super::{TokenStore, TokenStoreInternalError};
use async_trait::async_trait;
use crate::{Token, TokenID};
use redis_client::{aio::Connection, AsyncCommands, Client};
use std::sync::Arc;
use tokio::sync::Mutex;

pub use redis_client::RedisError as Error;

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
    async fn exists(&self, id: &TokenID) -> Result<bool, super::Error> {
        Ok(self
            .connection
            .lock()
            .await
            .exists::<_, bool>(id.to_string())
            .await?)
    }

    async fn add(&self, token: &Token) -> Result<(), super::Error> {
        self.connection
            .lock()
            .await
            .set(token.id().to_string(), token.to_string())
            .await?;

        Ok(())
    }

    async fn get(self: &Self, id: &TokenID) -> Result<Option<Token>, super::Error> {
        let token: Option<String> = self.connection.lock().await.get(id.to_string()).await?;
        let token: Option<Token> = match token.map(|token| Token::from_str(token.as_str())) {
            Some(token) => Some(token?),
            None => None,
        };
        Ok(token)
    }
}
