pub mod meta;
pub mod google_home;

use async_trait::async_trait;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::accessory::Accessory;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, strum::IntoStaticStr)]
pub enum Name {
    Master,
    Meta,
}

impl acu::MasterName for Name {
    fn master_name() -> Self {
        Self::Master
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Connected {
        accessory: Accessory,
    },
    Disconnected {
        accessory_id: accessory::ID,
    },

    Updated {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    },
}

impl acu::Message for Message {}

pub type Handle = acu::Handle<Message, Name>;

#[async_trait]
pub trait ControllerExt {
    async fn connected(&self, accessory: Accessory);
    async fn disconnected(&self, accessory_id: accessory::ID);
    async fn updated(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    );
}

#[async_trait]
impl ControllerExt for Handle {
    async fn connected(&self, accessory: Accessory) {
        self.sender.notify(Message::Connected { accessory }).await
    }

    async fn disconnected(&self, accessory_id: accessory::ID) {
        self.sender
            .notify(Message::Disconnected { accessory_id })
            .await
    }

    async fn updated(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) {
        self.sender
            .notify(Message::Updated {
                accessory_id,
                service_name,
                characteristic,
            })
            .await
    }
}

pub type MasterHandle = acu::BroadcasterMasterHandle<Message, Name>;
