mod memory;
mod redis;

pub use self::memory::{Error as MemoryTokenStoreError, MemoryTokenStore};
pub use self::redis::{Error as RedisTokenStoreError, RedisTokenStore};

use async_trait::async_trait;
use houseflow_types::token::{DecodeError, Token, TokenID};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: `{0}`")]
    InternalError(Box<dyn TokenStoreInternalError>),
    #[error("decoding token failed: `{0}`")]
    DecodeError(#[from] DecodeError),
}

pub trait TokenStoreInternalError: std::fmt::Debug + std::error::Error {}

impl<T: TokenStoreInternalError + 'static> From<T> for Error {
    fn from(v: T) -> Self {
        Self::InternalError(Box::new(v))
    }
}

#[async_trait]
pub trait TokenStore: Send + Sync {
    async fn exists(&self, id: &TokenID) -> Result<bool, Error>;

    async fn get(&self, id: &TokenID) -> Result<Option<Token>, Error>;

    async fn remove(&self, id: &TokenID) -> Result<bool, Error>;

    async fn add(&self, token: &Token) -> Result<(), Error>;
}
