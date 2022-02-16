pub use super::Handle;

use super::Message;
use super::Name;
use crate::providers;
use crate::providers::ProviderExt;
use acu::MasterExt;
use anyhow::Error;
use axum::AddExtensionLayer;
use axum::Json;
use futures::future::join_all;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::errors::ControllerError;
use houseflow_types::errors::ServerError;

pub fn new() -> Handle {
    let (sender, receiver) = acu::channel(8, Name::Master);
    let mut actor = MetaController { receiver };
    let handle = Handle { sender };
    tokio::spawn(async move { actor.run().await });
    handle
}

pub struct MetaController {
    receiver: acu::Receiver<Message, Name>,
}

impl MetaController {
    async fn run(&mut self) -> Result<(), Error> {
        while let Some(message) = self.receiver.recv().await {
            self.handle_message(message).await?;
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: Message) -> Result<(), anyhow::Error> {
        match message {
            Message::Connected { accessory: _ } => {}
            Message::Disconnected { accessory_id: _ } => {}
            Message::Updated {
                accessory_id: _,
                service_name: _,
                characteristic: _,
            } => {}
        };
        Ok(())
    }
}

pub fn app(handle: Handle) -> axum::Router {
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
        .layer(AddExtensionLayer::new(handle))
}

use axum::extract::Extension;
use axum::extract::Path;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;

pub async fn read_characteristic(
    Extension(master_provider): Extension<providers::MasterHandle>,
    Path((accessory_id, service_name, characteristic_name)): Path<(
        accessory::ID,
        ServiceName,
        CharacteristicName,
    )>,
) -> Result<Json<Characteristic>, ServerError> {
    let slaves: Vec<providers::Handle> = master_provider.slaves().await;
    let futures = slaves
        .iter()
        .map(|provider| async move { (provider, provider.is_connected(accessory_id).await) });
    let results = join_all(futures).await;
    let provider = results
        .into_iter()
        .find_map(|(provider, connected)| if connected { Some(provider) } else { None })
        .ok_or(ControllerError::AccessoryNotConnected)?;
    let characteristic = provider
        .read_characteristic(accessory_id, service_name, characteristic_name)
        .await
        .map_err(ControllerError::AccessoryError)?;
    Ok(Json(characteristic))
}

pub async fn write_characteristic(
    Extension(master_provider): Extension<providers::MasterHandle>,
    Path((accessory_id, service_name)): Path<(accessory::ID, ServiceName)>,
    Json(characteristic): Json<Characteristic>,
) -> Result<(), ServerError> {
    let slaves: Vec<providers::Handle> = master_provider.slaves().await;
    let futures = slaves
        .iter()
        .map(|provider| async move { (provider, provider.is_connected(accessory_id).await) });
    let results = join_all(futures).await;
    let provider = results
        .into_iter()
        .find_map(|(provider, connected)| if connected { Some(provider) } else { None })
        .ok_or(ControllerError::AccessoryNotConnected)?;
    provider
        .write_characteristic(accessory_id, service_name, characteristic)
        .await
        .map_err(ControllerError::AccessoryError)?;
    Ok(())
}
