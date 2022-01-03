use super::session;
use super::session::Session;
use crate::providers::EventSender;
use crate::providers::Provider;
use ::messages::prelude::*;
use anyhow::Error;
use async_trait::async_trait;
use axum::extract::ws;
use axum::extract::ws::WebSocket;
use futures::stream::SplitSink;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::HiveProvider as Config;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use std::collections::HashMap;

pub type Address = ::messages::prelude::Address<HiveProviderActor>;

pub struct HiveProviderActor {
    configured_accessories: Vec<Accessory>,
    sessions: HashMap<accessory::ID, session::Address>,
}

pub(super) mod messages {
    use super::*;

    #[derive(Debug)]
    pub struct Connected {
        pub sink: SplitSink<WebSocket, ws::Message>,
        pub accessory_id: accessory::ID,
    }

    #[derive(Debug)]
    pub struct Disconnected {
        pub accessory_id: accessory::ID,
    }

    #[derive(Debug)]
    pub struct WriteCharacteristic {
        pub accessory_id: accessory::ID,
        pub service_name: accessory::services::ServiceName,
        pub characteristic: accessory::characteristics::Characteristic,
    }

    #[derive(Debug)]
    pub struct ReadCharacteristic {
        pub accessory_id: accessory::ID,
        pub service_name: accessory::services::ServiceName,
        pub characteristic_name: accessory::characteristics::CharacteristicName,
    }

    #[derive(Debug)]
    pub struct GetAccessoryConfiguration {
        pub accessory_id: accessory::ID,
    }

    #[derive(Debug)]
    pub struct IsConnected {
        pub accessory_id: accessory::ID,
    }
}

impl HiveProviderActor {
    pub fn new(_config: Config, configured_accessories: Vec<Accessory>) -> Self {
        Self {
            configured_accessories,
            sessions: Default::default(),
        }
    }
}

#[async_trait]
impl Actor for HiveProviderActor {}

#[async_trait]
impl Handler<messages::Connected> for HiveProviderActor {
    type Result = Result<session::Address, Error>;

    async fn handle(
        &mut self,
        input: messages::Connected,
        _context: &Context<Self>,
    ) -> Self::Result {
        let session = {
            let actor = Session::new(input.accessory_id, input.sink);
            let context = Context::new();
            let address = context.address();
            tokio::spawn(context.run(actor));
            address
        };
        self.sessions.insert(input.accessory_id, session.clone());

        Ok(session)
    }
}

#[async_trait]
impl Handler<messages::Disconnected> for HiveProviderActor {
    type Result = ();

    async fn handle(
        &mut self,
        input: messages::Disconnected,
        _context: &Context<Self>,
    ) -> Self::Result {
        self.sessions.remove(&input.accessory_id);
    }
}

#[async_trait]
impl Handler<messages::WriteCharacteristic> for HiveProviderActor {
    type Result = Result<Result<(), accessory::Error>, Error>;

    async fn handle(
        &mut self,
        input: messages::WriteCharacteristic,
        _context: &Context<Self>,
    ) -> Self::Result {
        let oneshot = {
            let session = self.sessions.get_mut(&input.accessory_id).unwrap();
            session
                .send(session::messages::WriteCharacteristic {
                    service_name: input.service_name,
                    characteristic: input.characteristic,
                })
                .await??
        };
        Ok(oneshot.await?)
    }
}

#[async_trait]
impl Handler<messages::ReadCharacteristic> for HiveProviderActor {
    type Result = Result<Result<Characteristic, accessory::Error>, Error>;

    async fn handle(
        &mut self,
        input: messages::ReadCharacteristic,
        _context: &Context<Self>,
    ) -> Self::Result {
        let oneshot = {
            let session = self.sessions.get_mut(&input.accessory_id).unwrap();
            session
                .send(session::messages::ReadCharacteristic {
                    service_name: input.service_name,
                    characteristic_name: input.characteristic_name,
                })
                .await??
        };
        Ok(oneshot.await?)
    }
}

#[async_trait]
impl Handler<messages::GetAccessoryConfiguration> for HiveProviderActor {
    type Result = Option<Accessory>;

    async fn handle(
        &mut self,
        input: messages::GetAccessoryConfiguration,
        _context: &Context<Self>,
    ) -> Self::Result {
        self.configured_accessories
            .iter()
            .find(|accessory| accessory.id == input.accessory_id)
            .cloned()
    }
}

#[async_trait]
impl Handler<messages::IsConnected> for HiveProviderActor {
    type Result = bool;

    async fn handle(
        &mut self,
        input: messages::IsConnected,
        _context: &Context<Self>,
    ) -> Self::Result {
        self.sessions.contains_key(&input.accessory_id)
    }
}

pub struct HiveProvider {
    address: Address,
    events: EventSender,
}

impl HiveProvider {
    pub async fn new(
        _config: Config,
        configured_accessories: Vec<Accessory>,
        events: EventSender,
    ) -> Result<Self, anyhow::Error> {
        let actor = HiveProviderActor::new(_config, configured_accessories);
        let address = actor.spawn();

        Ok(Self {
            address,
            events,
        })
    }
}

#[async_trait]
impl Provider for HiveProvider {
    async fn run(&self) -> Result<(), Error> {
        use std::net::IpAddr;
        use std::net::Ipv4Addr;
        use std::net::SocketAddr;

        let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 8080);
        let app = super::app(self.address.clone(), self.events.clone());
        axum::Server::bind(&socket_address)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    async fn write_characteristic(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic: &Characteristic,
    ) -> Result<Result<(), accessory::Error>, Error> {
        self.address
            .clone()
            .send(messages::WriteCharacteristic {
                accessory_id: *accessory_id,
                service_name: service_name.to_owned(),
                characteristic: characteristic.to_owned(),
            })
            .await
            .unwrap()
    }

    async fn read_characteristic(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceName,
        characteristic_name: &CharacteristicName,
    ) -> Result<Result<Characteristic, accessory::Error>, Error> {
        self.address
            .clone()
            .send(messages::ReadCharacteristic {
                accessory_id: *accessory_id,
                service_name: service_name.to_owned(),
                characteristic_name: characteristic_name.to_owned(),
            })
            .await
            .unwrap()
    }

    async fn is_connected(&self, accessory_id: &accessory::ID) -> bool {
        self.address
            .clone()
            .send(messages::IsConnected {
                accessory_id: *accessory_id,
            })
            .await
            .unwrap()
    }

    fn name(&self) -> &'static str {
        "mijia"
    }
}
