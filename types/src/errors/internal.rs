use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("token blacklist error: {0}")]
    TokenBlacklistError(String),

    #[error("other: {0}")]
    Other(String),
}
