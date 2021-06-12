use async_trait::async_trait;

mod postgres;
mod memory;
pub use postgres::{PostgresConfig, PostgresDatabase, PostgresError};
pub use memory::{MemoryDatabase, MemoryDatabaseError};

pub trait DatabaseInternalError: std::fmt::Debug + std::error::Error {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: `{0}`")]
    InternalError(Box<dyn DatabaseInternalError>),

    #[error("Query did not modify anything")]
    NotModified,
}

impl<T: DatabaseInternalError + 'static> From<T> for Error {
    fn from(v: T) -> Self {
        Self::InternalError(Box::new(v))
    }
}

use types::{Device, DeviceID, User, UserID};

#[async_trait]
pub trait Database: Send + Sync {
    async fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error>;

    async fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error>;
    async fn add_user(&self, user: &User) -> Result<(), Error>;
    async fn delete_user(&self, user_id: &UserID) -> Result<(), Error>;
}
