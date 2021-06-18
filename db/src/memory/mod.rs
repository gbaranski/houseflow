use crate::{DatabaseInternalError, Error};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use types::{Device, DeviceID, DevicePermission, User, UserID};

#[derive(Debug, thiserror::Error)]
pub enum InternalError {}

impl DatabaseInternalError for InternalError {}

#[derive(Clone, Default)]
pub struct Database {
    users: Arc<Mutex<HashMap<UserID, User>>>,
    user_devices: Arc<Mutex<HashMap<(UserID, DeviceID), DevicePermission>>>,
    devices: Arc<Mutex<HashMap<DeviceID, Device>>>,
}

impl Database {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl crate::Database for Database {
    async fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error> {
        Ok(self.devices.lock().await.get(device_id).cloned())
    }

    async fn add_device(&self, device: &Device) -> Result<(), Error> {
        let mut devices = self.devices.lock().await;
        devices.insert(device.id.clone(), device.clone());
        Ok(())
    }

    async fn get_user_devices(
        &self,
        user_id: &UserID,
        permission: &DevicePermission,
    ) -> Result<Vec<Device>, Error> {
        let user_devices = self.user_devices.lock().await;
        let devices = self.devices.lock().await;
        let user_devices: Vec<Device> = user_devices
            .iter()
            .filter_map(|((d_user_id, d_device_id), d_permission)| {
                if d_user_id == user_id
                    && d_permission.read >= permission.read
                    && d_permission.write >= permission.write
                    && d_permission.execute >= permission.execute
                {
                    Some(
                        devices
                            .iter()
                            .find(|device| device.0 == d_device_id)
                            .unwrap()
                            .1
                            .clone(),
                    )
                } else {
                    None
                }
            })
            .collect();

        Ok(user_devices)
    }

    async fn check_user_device_permission(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
        permission: &DevicePermission,
    ) -> Result<bool, Error> {
        let user_devices = self.user_devices.lock().await;
        Ok(user_devices
            .iter()
            .any(|((d_user_id, d_device_id), d_permission)| {
                d_user_id == user_id
                    && d_device_id == device_id
                    && d_permission.read >= permission.read
                    && d_permission.write >= permission.write
                    && d_permission.execute >= permission.execute
            }))
    }

    async fn add_user_device(
        &self,
        device_id: &DeviceID,
        user_id: &UserID,
        permission: &DevicePermission,
    ) -> Result<(), Error> {
        let mut user_devices = self.user_devices.lock().await;
        user_devices.insert((user_id.clone(), device_id.clone()), permission.clone());

        Ok(())
    }

    async fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error> {
        Ok(self.users.lock().await.get(user_id).cloned())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let users = self.users.lock().await;
        let user = users.iter().find_map(|(_, user)| {
            if user.email == *email {
                Some(user.clone())
            } else {
                None
            }
        });
        Ok(user)
    }

    async fn add_user(&self, user: &User) -> Result<(), Error> {
        let mut users = self.users.lock().await;
        users.insert(user.id.clone(), user.clone());
        Ok(())
    }

    async fn delete_user(&self, user_id: &UserID) -> Result<(), Error> {
        self.users.lock().await.remove(user_id);
        Ok(())
    }
}
