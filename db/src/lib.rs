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

use houseflow_types::{
    Device, DeviceID, Room, RoomID, Structure, StructureID, User, UserID, UserStructure,
};

pub trait Database: Send + Sync {
    fn add_user(&self, user: &User) -> Result<(), Error>;
    fn add_admin(&self, user_id: &UserID) -> Result<(), Error>;
    fn add_structure(&self, structure: &Structure) -> Result<(), Error>;
    fn add_room(&self, room: &Room) -> Result<(), Error>;
    fn add_device(&self, device: &Device) -> Result<(), Error>;
    fn add_user_structure(&self, user_structure: &UserStructure) -> Result<(), Error>;

    fn delete_user(&self, user_id: &UserID) -> Result<bool, Error>;
    fn delete_admin(&self, user_id: &UserID) -> Result<bool, Error>;
    fn delete_structure(&self, structure_id: &StructureID) -> Result<bool, Error>;
    fn delete_room(&self, room_id: &RoomID) -> Result<bool, Error>;
    fn delete_device(&self, device_id: &DeviceID) -> Result<bool, Error>;
    fn delete_user_structure(
        &self,
        structure_id: &StructureID,
        user_id: &UserID,
    ) -> Result<bool, Error>;

    fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error>;
    fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error>;
    fn get_structure(&self, structure_id: &StructureID) -> Result<Option<Structure>, Error>;
    fn get_room(&self, room_id: &RoomID) -> Result<Option<Room>, Error>;
    fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error>;
    fn get_user_devices(&self, user_id: &UserID) -> Result<Vec<Device>, Error>;

    fn check_user_device_access(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
    ) -> Result<bool, Error>;

    fn check_user_admin(&self, user_id: &UserID) -> Result<bool, Error>;
}

impl From<Error> for houseflow_types::errors::ServerError {
    fn from(val: Error) -> Self {
        houseflow_types::errors::InternalError::DatabaseError(val.to_string()).into()
    }
}
