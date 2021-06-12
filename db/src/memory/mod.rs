use crate::{Database, DatabaseInternalError, Error};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use types::{Device, DeviceID, User, UserID};

#[derive(Debug, thiserror::Error)]
pub enum MemoryDatabaseError {}

impl DatabaseInternalError for MemoryDatabaseError {}

#[derive(Clone, Default)]
pub struct MemoryDatabase {
    users: Arc<Mutex<HashMap<UserID, User>>>,
    devices: Arc<Mutex<HashMap<DeviceID, Device>>>,
}

impl MemoryDatabase {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl Database for MemoryDatabase {
    async fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error> {
        Ok(self.devices.lock().await.get(device_id).cloned())
    }

    async fn add_device(&self, device: &Device) -> Result<(), Error> {
        let mut devices = self.devices.lock().await;
        devices.insert(device.id.clone(), device.clone());
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
