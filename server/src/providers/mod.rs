pub mod dummy;
pub mod lighthouse;

pub use dummy::DummyProvider;
pub use lighthouse::LighthouseProvider;

use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum Name {
    Master,
    Dummy,
    Lighthouse,
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
    GetAccessories {
        respond_to: oneshot::Sender<Vec<accessory::ID>>,
    },
    IsConnected {
        accessory_id: accessory::ID,
        respond_to: oneshot::Sender<bool>,
    },
}

impl acu::Message for Message {}

use async_trait::async_trait;

#[async_trait]
pub trait ProviderExt {
    async fn write_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error>;
    async fn read_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error>;
    async fn get_accessories(&self) -> Vec<accessory::ID>;
    async fn is_connected(&self, accessory_id: accessory::ID) -> bool;
}

pub type Handle = acu::Handle<Message, Name>;

#[async_trait]
impl ProviderExt for Handle {
    async fn write_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error> {
        self.sender
            .call_with(|respond_to| Message::WriteCharacteristic {
                accessory_id,
                service_name,
                characteristic,
                respond_to,
            })
            .await
    }

    async fn read_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error> {
        self.sender
            .call_with(|respond_to| Message::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            })
            .await
    }

    async fn get_accessories(&self) -> Vec<accessory::ID> {
        self.sender
            .call_with(|respond_to| Message::GetAccessories { respond_to })
            .await
    }

    async fn is_connected(&self, accessory_id: accessory::ID) -> bool {
        self.sender
            .call_with(|respond_to| Message::IsConnected {
                accessory_id,
                respond_to,
            })
            .await
    }
}

pub type MasterHandle = acu::MasterHandle<Message, Name>;
