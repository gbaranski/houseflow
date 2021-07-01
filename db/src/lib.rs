use async_trait::async_trait;

pub mod memory;
pub mod postgres;

pub trait DatabaseInternalError: std::fmt::Debug + std::error::Error + Send + Sync {}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: `{0}`")]
    InternalError(Box<dyn DatabaseInternalError>),

    #[error("Query did not modify anything")]
    NotModified,

    #[error("Row already exists")]
    AlreadyExists,
}

impl<T: DatabaseInternalError + 'static> From<T> for Error {
    fn from(v: T) -> Self {
        Self::InternalError(Box::new(v))
    }
}

use houseflow_types::{Device, DeviceID, Room, Structure, User, UserID, UserStructure};

#[async_trait]
pub trait Database: Send + Sync {
    async fn add_structure(&self, structure: &Structure) -> Result<(), Error>;

    async fn add_room(&self, room: &Room) -> Result<(), Error>;

    async fn add_device(&self, device: &Device) -> Result<(), Error>;

    async fn add_user_structure(&self, user_structure: &UserStructure) -> Result<(), Error>;

    async fn add_user(&self, user: &User) -> Result<(), Error>;

    async fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error>;

    async fn get_user_devices(&self, user_id: &UserID) -> Result<Vec<Device>, Error>;

    async fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error>;

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error>;

    async fn check_user_device_access(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
    ) -> Result<bool, Error>;

    async fn check_user_device_manager_access(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
    ) -> Result<bool, Error>;

    async fn check_user_admin(&self, user_id: &UserID) -> Result<bool, Error>;

    async fn delete_user(&self, user_id: &UserID) -> Result<(), Error>;
}
