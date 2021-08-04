pub mod sled;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use houseflow_types::token::RefreshTokenID;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sled error: {0}")]
    SledError(#[from] ::sled::Error),

    #[error("invalid data {0}")]
    InvalidData(String),
}

#[async_trait]
pub trait TokenStore: Send + Sync {
    async fn exists(&self, id: &RefreshTokenID) -> Result<bool, Error>;

    async fn remove(&self, id: &RefreshTokenID) -> Result<bool, Error>;

    async fn add(
        &self,
        id: &RefreshTokenID,
        expire_at: Option<&DateTime<Utc>>,
    ) -> Result<(), Error>;
}

impl From<Error> for houseflow_types::ServerError {
    fn from(val: Error) -> Self {
        houseflow_types::ServerError::InternalError(
            houseflow_types::InternalServerError::TokenStoreError(val.to_string()),
        )
    }
}
