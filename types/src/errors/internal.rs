use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("token store error: {0}")]
    TokenStoreError(String),

    #[error("database error: {0}")]
    DatabaseError(String),

    #[error("other: {0}")]
    Other(String),
}
