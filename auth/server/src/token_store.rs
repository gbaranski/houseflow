use houseflow_token::{DecodeError, Token, TokenID};
use redis::{aio::Connection, AsyncCommands, Client};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct TokenStore {
    connection: Arc<Mutex<Connection>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error ocurred with Redis: `{0}`")]
    RedisError(#[from] redis::RedisError),

    #[error("decoding token failed: `{0}`")]
    DecodeError(#[from] DecodeError),
}

impl TokenStore {
    pub async fn new() -> Result<Self, Error> {
        let client = Client::open("redis://127.0.0.1")?;
        let connection = client.get_tokio_connection().await?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub async fn exists(&self, id: &TokenID) -> Result<bool, Error> {
        Ok(self
            .connection
            .lock()
            .await
            .exists::<_, bool>(id.to_string())
            .await?)
    }

    pub async fn set(&self, token: Token) -> Result<(), Error> {
        self.connection
            .lock()
            .await
            .set(token.payload.id.to_string(), token.into_base64())
            .await?;

        Ok(())
    }
}
