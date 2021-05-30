use async_trait::async_trait;
use houseflow_token::{Token, TokenID};

mod memory;
mod redis;
pub use memory::{Error as MemoryTokenStoreError, MemoryTokenStore};
pub use redis::{Error as RedisTokenStoreError, RedisTokenStore};

#[derive(Debug, thiserror::Error)]
pub enum Error<IE>
where
    IE: std::error::Error + 'static,
{
    #[error("internal error: `{0}`")]
    InternalError(#[from] IE),
    //     #[error("decoding token failed: `{0}`")]
    //     DecodeError(#[from] DecodeError),
}

#[async_trait]
pub trait TokenStore<IE: std::fmt::Debug + std::error::Error>: Clone {
    async fn exists(&self, id: &TokenID) -> Result<bool, Error<IE>>;
    async fn set(&self, token: &Token) -> Result<(), Error<IE>>;
}
