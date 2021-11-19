use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
#[error("{description}")]
pub struct Error {
    pub description: String,
}

#[cfg(feature = "token")]
impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self {
            description: e.to_string(),
        }
    }
}
