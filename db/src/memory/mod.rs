use crate::{DatabaseInternalError, Error};
use async_trait::async_trait;
use houseflow_types::{Device, DeviceID, Room, Structure, User, UserID, UserStructure};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error)]
pub enum InternalError {}

struct Admin {
    user_id: UserID,
}

impl DatabaseInternalError for InternalError {}

type ThreadVec<T> = Arc<Mutex<Vec<T>>>;

#[derive(Clone, Default)]
pub struct Database {
    users: ThreadVec<User>,
    admins: ThreadVec<Admin>,
    structures: ThreadVec<Structure>,
    rooms: ThreadVec<Room>,
    user_structures: ThreadVec<UserStructure>,
    devices: ThreadVec<Device>,
}

impl Database {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl crate::Database for Database {
    async fn add_structure(&self, structure: &Structure) -> Result<(), Error> {
        let mut structures = self.structures.lock().await;
        structures.push(structure.clone());

        Ok(())
    }

    async fn add_room(&self, room: &Room) -> Result<(), Error> {
        let mut rooms = self.rooms.lock().await;
        rooms.push(room.clone());

        Ok(())
    }

    async fn add_device(&self, device: &Device) -> Result<(), Error> {
        let mut devices = self.devices.lock().await;
        devices.push(device.clone());
        Ok(())
    }

    async fn add_user_structure(&self, user_structure: &UserStructure) -> Result<(), Error> {
        let mut user_structures = self.user_structures.lock().await;
        user_structures.push(user_structure.clone());
        Ok(())
    }

    async fn add_user(&self, user: &User) -> Result<(), Error> {
        let mut users = self.users.lock().await;
        users.push(user.clone());
        Ok(())
    }

    async fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error> {
        Ok(self
            .devices
            .lock()
            .await
            .iter()
            .find(|device| device.id == *device_id)
            .cloned())
    }

    async fn get_user_devices(&self, user_id: &UserID) -> Result<Vec<Device>, Error> {
        let user_structures = self.user_structures.lock().await;
        let rooms = self.rooms.lock().await;
        let devices = self.devices.lock().await;
        let user_devices = user_structures
            .iter()
            .filter(|user_structure| user_structure.user_id == *user_id)
            .filter_map(|user_structure| {
                rooms
                    .iter()
                    .find(|room| room.structure_id == user_structure.structure_id)
            })
            .filter_map(|room| devices.iter().find(|device| device.room_id == room.id))
            .cloned()
            .collect::<Vec<_>>();

        Ok(user_devices)
    }

    async fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error> {
        let user = self
            .users
            .lock()
            .await
            .iter()
            .find(|user| user.id == *user_id)
            .cloned();
        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let users = self.users.lock().await;
        let user = users.iter().find(|user| user.email == *email).cloned();
        Ok(user)
    }

    async fn check_user_device_access(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
    ) -> Result<bool, Error> {
        let user_structures = self.user_structures.lock().await;
        let rooms = self.rooms.lock().await;
        let devices = self.devices.lock().await;
        let have_access = user_structures
            .iter()
            .filter(|user_structure| user_structure.user_id == *user_id)
            .filter_map(|user_structure| {
                rooms
                    .iter()
                    .find(|room| room.structure_id == user_structure.structure_id)
            })
            .any(|room| {
                devices
                    .iter()
                    .filter(|device| device.room_id == room.id)
                    .any(|device| device.id == *device_id)
            });

        Ok(have_access)
    }

    async fn check_user_device_manager_access(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
    ) -> Result<bool, Error> {
        let user_structures = self.user_structures.lock().await;
        let rooms = self.rooms.lock().await;
        let devices = self.devices.lock().await;
        let have_access = user_structures
            .iter()
            .filter(|user_structure| {
                user_structure.is_manager && user_structure.user_id == *user_id
            })
            .filter_map(|user_structure| {
                rooms
                    .iter()
                    .find(|room| room.structure_id == user_structure.structure_id)
            })
            .any(|room| {
                devices
                    .iter()
                    .filter(|device| device.room_id == room.id)
                    .any(|device| device.id == *device_id)
            });

        Ok(have_access)
    }

    async fn check_user_admin(
        &self,
        user_id: &UserID,
    ) -> Result<bool, Error> {
        let admins = self.admins.lock().await;
        let is_admin = admins.iter().any(|admin| admin.user_id == *user_id);

        Ok(is_admin)
    }

    async fn delete_user(&self, user_id: &UserID) -> Result<(), Error> {
        let mut users = self.users.lock().await;
        let pos = users
            .iter()
            .position(|user| user.id == *user_id)
            .ok_or(Error::NotModified)?;
        users.remove(pos);
        Ok(())
    }
}
