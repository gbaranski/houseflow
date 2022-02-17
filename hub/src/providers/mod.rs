pub mod hive;
pub mod mijia;

use acu::MasterExt;
use async_trait::async_trait;
use futures::future;
use houseflow_config::hub::Accessory;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::accessory::{Error, ID};
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum Name {
    Master,
    Hive,
    Mijia,
}

impl acu::MasterName for Name {
    fn master_name() -> Self {
        Self::Master
    }
}

#[derive(Debug)]
pub enum Message {
    ReadCharacteristic {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
        respond_to: oneshot::Sender<Result<Characteristic, accessory::Error>>,
    },
    WriteCharacteristic {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
        respond_to: oneshot::Sender<Result<(), accessory::Error>>,
    },
    GetAccessoryConfiguration {
        accessory_id: accessory::ID,
        respond_to: oneshot::Sender<Option<Accessory>>,
    },
    IsConnected {
        accessory_id: accessory::ID,
        respond_to: oneshot::Sender<bool>,
    },
}

impl acu::Message for Message {}

pub type Handle = acu::Handle<Message, Name>;

#[async_trait]
pub trait ProviderExt {
    async fn read_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error>;
    async fn write_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error>;
    async fn is_connected(&self, accessory_id: accessory::ID) -> bool;
    async fn get_accessory_configuration(&self, accessory_id: accessory::ID) -> Option<Accessory>;
}

#[async_trait]
impl ProviderExt for Handle {
    async fn read_characteristic(
        &self,
        accessory_id: ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, Error> {
        self.sender
            .call_with(|respond_to| Message::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            })
            .await
    }

    async fn write_characteristic(
        &self,
        accessory_id: ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), Error> {
        self.sender
            .call_with(|respond_to| Message::WriteCharacteristic {
                accessory_id,
                service_name,
                characteristic,
                respond_to,
            })
            .await
    }

    async fn is_connected(&self, accessory_id: ID) -> bool {
        self.sender
            .call_with(|respond_to| Message::IsConnected {
                accessory_id,
                respond_to,
            })
            .await
    }

    async fn get_accessory_configuration(&self, accessory_id: ID) -> Option<Accessory> {
        self.sender
            .call_with(|respond_to| Message::GetAccessoryConfiguration {
                accessory_id,
                respond_to,
            })
            .await
    }
}

pub type MasterHandle = acu::MasterHandle<Message, Name>;

#[async_trait]
impl ProviderExt for MasterHandle {
    async fn read_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error> {
        let slaves = self.slaves().await;
        let futures = slaves
            .iter()
            .map(|handle| async move { (handle, handle.is_connected(accessory_id).await) });
        let results = future::join_all(futures).await;
        let slave = results
            .iter()
            .find_map(|(handle, connected)| if *connected { Some(handle) } else { None })
            .ok_or(accessory::Error::NotConnected)?;
        slave
            .read_characteristic(accessory_id, service_name, characteristic_name)
            .await
    }

    async fn write_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error> {
        let slaves = self.slaves().await;
        let futures = slaves
            .iter()
            .map(|handle| async move { (handle, handle.is_connected(accessory_id).await) });
        let results = future::join_all(futures).await;
        let slave = results
            .iter()
            .find_map(|(handle, connected)| if *connected { Some(handle) } else { None })
            .ok_or(accessory::Error::NotConnected)?;
        slave
            .write_characteristic(accessory_id, service_name, characteristic)
            .await
    }

    async fn is_connected(&self, accessory_id: accessory::ID) -> bool {
        let slaves = self.slaves().await;
        let futures = slaves
            .iter()
            .map(|handle| handle.is_connected(accessory_id));
        let results = future::join_all(futures).await;
        results.iter().any(|connected| *connected)
    }

    async fn get_accessory_configuration(&self, accessory_id: accessory::ID) -> Option<Accessory> {
        let slaves = self.slaves().await;
        let slave = slaves.first().unwrap(); // TODO: Do something different maybe?
        slave.get_accessory_configuration(accessory_id).await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum SessionName {
    HiveSession,
    MijiaSession,
}

impl acu::Name for SessionName {}

#[derive(Debug)]
pub enum SessionMessage {
    ReadCharacteristic {
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
        respond_to: oneshot::Sender<oneshot::Receiver<Result<Characteristic, accessory::Error>>>,
    },
    WriteCharacteristic {
        service_name: ServiceName,
        characteristic: Characteristic,
        respond_to: oneshot::Sender<oneshot::Receiver<Result<(), accessory::Error>>>,
    },
}

impl acu::Message for SessionMessage {}

pub type SessionHandle = acu::Handle<SessionMessage, SessionName>;

#[async_trait]
pub trait SessionExt {
    async fn read_characteristic(
        &self,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error>;

    async fn write_characteristic(
        &self,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error>;
}

#[async_trait]
impl SessionExt for SessionHandle {
    async fn read_characteristic(
        &self,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error> {
        self.sender
            .call_with(|respond_to| SessionMessage::ReadCharacteristic {
                service_name,
                characteristic_name,
                respond_to,
            })
            .await
            .await
            .unwrap()
    }

    async fn write_characteristic(
        &self,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error> {
        self.sender
            .call_with(|respond_to| SessionMessage::WriteCharacteristic {
                service_name,
                characteristic,
                respond_to,
            })
            .await
            .await
            .unwrap()
    }
}
