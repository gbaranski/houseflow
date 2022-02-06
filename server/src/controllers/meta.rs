use super::Handle;
use super::Message;
use super::Name;
use crate::providers;
use anyhow::Error;
use axum::Json;
use houseflow_config::server::controllers::Meta as Config;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::errors::ControllerError;
use houseflow_types::errors::ServerError;

#[derive(Debug)]
pub enum MetaMessage {}

#[derive(Debug, Clone)]
pub struct MetaHandle {
    sender: acu::Sender<MetaMessage>,
    handle: Handle,
}

impl std::ops::Deref for MetaHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl From<MetaHandle> for Handle {
    fn from(val: MetaHandle) -> Self {
        val.handle
    }
}

pub struct MetaController {
    provider_receiver: acu::Receiver<Message>,
    meta_receiver: acu::Receiver<MetaMessage>,
    provider: providers::Handle,
}

impl MetaController {
    pub fn create(provider: providers::Handle, _config: Config) -> MetaHandle {
        let (provider_sender, provider_receiver) = acu::channel(8, Name::Meta.into());
        let (meta_sender, meta_receiver) = acu::channel(8, Name::Meta.into());
        let mut actor = Self {
            provider_receiver,
            meta_receiver,
            provider,
        };

        let handle = Handle::new(Name::Meta, provider_sender);
        tokio::spawn(async move { actor.run().await });
        MetaHandle {
            sender: meta_sender,
            handle,
        }
    }

    async fn run(&mut self) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(message) = self.provider_receiver.recv() => {
                    self.handle_controller_message(message).await?;
                },
                Some(message) = self.meta_receiver.recv() => {
                    self.handle_meta_message(message).await?
                },
                else => break,
            }
        }
        Ok(())
    }

    async fn handle_controller_message(&mut self, message: Message) -> Result<(), anyhow::Error> {
        match message {
            Message::Connected {
                accessory: _,
            } => {},
            Message::Disconnected { accessory_id: _ } => {},
            Message::Updated {
                accessory_id: _,
                service_name: _,
                characteristic: _,
            } => {},
        };
        Ok(())
    }

    async fn handle_meta_message(&mut self, message: MetaMessage) -> Result<(), anyhow::Error> {
        match message {};
        Ok(())
    }
}

pub fn app() -> axum::Router {
    use axum::routing::get;
    use axum::routing::post;

    axum::Router::new()
        .route(
            "/characteristic/:accessory_id/:service_name/:characteristic_name",
            get(read_characteristic),
        )
        .route(
            "/characteristic/:accessory_id/:service_name",
            post(write_characteristic),
        )
}

use axum::extract::Extension;
use axum::extract::Path;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;

pub async fn read_characteristic(
    Extension(provider): Extension<providers::Handle>,
    Path((accessory_id, service_name, characteristic_name)): Path<(
        accessory::ID,
        ServiceName,
        CharacteristicName,
    )>,
) -> Result<Json<Characteristic>, ServerError> {
    let characteristic = provider
        .read_characteristic(accessory_id, service_name, characteristic_name)
        .await
        .map_err(ControllerError::AccessoryError)?;
    Ok(Json(characteristic))
}

pub async fn write_characteristic(
    Extension(provider): Extension<providers::Handle>,
    Path((accessory_id, service_name)): Path<(accessory::ID, ServiceName)>,
    Json(characteristic): Json<Characteristic>,
) -> Result<(), ServerError> {
    provider
        .write_characteristic(accessory_id, service_name, characteristic)
        .await
        .map_err(ControllerError::AccessoryError)?;
    Ok(())
}
