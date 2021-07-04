mod memory;
mod redis;

pub use self::memory::{Error as MemoryTokenStoreError, MemoryTokenStore};
pub use self::redis::{Error as RedisTokenStoreError, RedisTokenStore};

use async_trait::async_trait;
use houseflow_types::token::RefreshTokenID;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: `{0}`")]
    InternalError(Box<dyn TokenStoreInternalError>),
}

pub trait TokenStoreInternalError: std::fmt::Debug + std::error::Error {}

impl<T: TokenStoreInternalError + 'static> From<T> for Error {
    fn from(v: T) -> Self {
        Self::InternalError(Box::new(v))
    }
}

#[async_trait]
pub trait TokenStore: Send + Sync {
    async fn exists(&self, id: &RefreshTokenID) -> Result<bool, Error>;

    async fn remove(&self, id: &RefreshTokenID) -> Result<bool, Error>;

    async fn add(&self, id: &RefreshTokenID) -> Result<(), Error>;
}

impl From<Error> for houseflow_types::InternalServerError {
    fn from(val: Error) -> Self {
        houseflow_types::InternalServerError::TokenStoreError(val.to_string())
    }
}

impl Error {
    pub fn into_internal_server_error(self) -> houseflow_types::InternalServerError {
        houseflow_types::InternalServerError::TokenStoreError(self.to_string())
    }
}
