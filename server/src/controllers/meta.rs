use super::Handle;
use super::Message;
use super::Name;
use crate::providers;
use crate::State;
use anyhow::Error;
use axum::Json;
use houseflow_config::server::controllers::Meta as Config;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::errors::ControllerError;
use houseflow_types::errors::ServerError;

pub struct MetaController {
    provider_receiver: acu::Receiver<Message>,
}

impl MetaController {
    pub fn create(_provider: providers::Handle, _config: Config) -> Handle {
        let (provider_sender, provider_receiver) = acu::channel(8, Name::Meta.into());
        let mut actor = Self { provider_receiver };

        let handle = Handle::new(Name::Meta, provider_sender);
        tokio::spawn(async move { actor.run().await });
        handle
    }

    async fn run(&mut self) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(message) = self.provider_receiver.recv() => {
                    self.handle_controller_message(message).await?;
                },
                else => break,
            }
        }
        Ok(())
    }

    async fn handle_controller_message(&mut self, message: Message) -> Result<(), anyhow::Error> {
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
    Extension(state): Extension<State>,
    Path((accessory_id, service_name, characteristic_name)): Path<(
        accessory::ID,
        ServiceName,
        CharacteristicName,
    )>,
) -> Result<Json<Characteristic>, ServerError> {
    let characteristic = state
        .provider
        .read_characteristic(accessory_id, service_name, characteristic_name)
        .await
        .map_err(ControllerError::AccessoryError)?;
    Ok(Json(characteristic))
}

pub async fn write_characteristic(
    Extension(state): Extension<State>,
    Path((accessory_id, service_name)): Path<(accessory::ID, ServiceName)>,
    Json(characteristic): Json<Characteristic>,
) -> Result<(), ServerError> {
    state
        .provider
        .write_characteristic(accessory_id, service_name, characteristic)
        .await
        .map_err(ControllerError::AccessoryError)?;
    Ok(())
}
