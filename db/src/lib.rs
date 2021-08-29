#[cfg(feature = "sqlite")]
pub mod sqlite;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "sqlite")]
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[cfg(feature = "refinery")]
    #[error("sqlite error: {0}")]
    Refinery(#[from] refinery::Error),

    #[cfg(feature = "refinery")]
    #[error("sqlite error: {0}")]
    PoolError(#[from] r2d2::Error),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Query did not modify anything")]
    NotModified,

    #[error("Row already exists")]
    AlreadyExists,
}

use houseflow_types::User;
use houseflow_types::UserID;

pub trait Database: Send + Sync {
    fn add_user(&self, user: &User) -> Result<(), Error>;
    fn add_admin(&self, user_id: &UserID) -> Result<(), Error>;

    fn delete_user(&self, user_id: &UserID) -> Result<bool, Error>;
    fn delete_admin(&self, user_id: &UserID) -> Result<bool, Error>;

    fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error>;
    fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error>;

    fn check_user_admin(&self, user_id: &UserID) -> Result<bool, Error>;
}

impl From<Error> for houseflow_types::errors::ServerError {
    fn from(val: Error) -> Self {
        houseflow_types::errors::InternalError::DatabaseError(val.to_string()).into()
    }
}
