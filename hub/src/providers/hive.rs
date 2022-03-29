pub use super::Handle;

use super::Message;
use super::SessionMessage;
use crate::controllers;
use crate::controllers::ControllerExt;
use crate::providers;
use crate::providers::ProviderExt;
use crate::ConfiguredAccessories;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::Extension;
use axum::extract::TypedHeader;
use axum::headers;
use axum::http::StatusCode;
use axum::response::Response;
use ezsockets::SessionExt;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::HiveProvider as Config;
use houseflow_types::accessory;
use houseflow_types::hive;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct HiveProvider {
    controller: controllers::MasterHandle,
    sessions: HashMap<accessory::ID, Session>,
    configured_accessories: ConfiguredAccessories,
}

#[derive(Debug)]
pub struct Args {
    accessory: Accessory,
}

type Server = ezsockets::Server<HiveProvider>;
type Session = ezsockets::Session<accessory::ID, SessionMessage>;

pub fn new(
    _config: Config,
    controller: controllers::MasterHandle,
    configured_accessories: ConfiguredAccessories,
) -> Server {
    let (server, _) = ezsockets::Server::create(|_| HiveProvider {
        controller,
        configured_accessories,
        sessions: Default::default(),
    });
    server
}

#[async_trait]
impl ezsockets::ServerExt for HiveProvider {
    type Session = HiveSession;
    type Params = Message;

    async fn accept(
        &mut self,
        socket: ezsockets::Socket,
        _address: std::net::SocketAddr,
        args: <Self::Session as SessionExt>::Args,
    ) -> Result<Session, ezsockets::Error> {
        let Args { accessory } = args;
        let session = Session::create(
            |session| HiveSession {
                session,
                accessory_id: accessory.id,
                controller: self.controller.clone(),
                characteristic_write_results: Default::default(),
                characteristic_read_results: Default::default(),
            },
            accessory.id,
            socket,
        );
        self.sessions.insert(accessory.id, session.clone());
        self.controller.connected(accessory).await;
        Ok(session)
    }

    async fn disconnected(
        &mut self,
        id: <Self::Session as ezsockets::SessionExt>::ID,
    ) -> Result<(), ezsockets::Error> {
        self.sessions.remove(&id).unwrap();
        Ok(())
    }

    async fn call(&mut self, params: Self::Params) -> Result<(), ezsockets::Error> {
        match params {
            Message::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            } => {
                let session = self.sessions.get(&accessory_id).unwrap();
                let result = session
                    .call_with(|respond_to| SessionMessage::ReadCharacteristic {
                        service_name,
                        characteristic_name,
                        respond_to,
                    })
                    .await
                    .await
                    .unwrap();
                respond_to.send(result).unwrap();
            }
            Message::WriteCharacteristic {
                accessory_id,
                service_name,
                characteristic,
                respond_to,
            } => {
                let session = self.sessions.get(&accessory_id).unwrap();
                let result = session
                    .call_with(|respond_to| SessionMessage::WriteCharacteristic {
                        service_name,
                        characteristic,
                        respond_to,
                    })
                    .await
                    .await
                    .unwrap();
                respond_to.send(result).unwrap();
            }
            Message::GetAccessoryConfiguration {
                accessory_id,
                respond_to,
            } => {
                let accessory_configuration = self
                    .configured_accessories
                    .load()
                    .iter()
                    .find(|accessory| accessory.id == accessory_id)
                    .cloned();
                respond_to.send(accessory_configuration).unwrap();
            }
            Message::IsConnected {
                accessory_id,
                respond_to,
            } => {
                let is_connected = self.sessions.get(&accessory_id).is_some();
                respond_to.send(is_connected).unwrap();
            }
        };
        Ok(())
    }
}

pub fn app(
    server: Server,
    configured_accessories: ConfiguredAccessories,
    provider: providers::MasterHandle,
) -> axum::Router {
    use axum::routing::get;

    axum::Router::new()
        .route("/websocket", get(websocket_handler))
        .layer(Extension(server))
        .layer(Extension(configured_accessories))
        .layer(Extension(provider))
}

pub struct DeviceCredentials(accessory::ID, accessory::Password);

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "error", content = "description")]
pub enum ConnectError {
    InvalidAuthorizationHeader(String),
    AccessoryNotFound,
    AccessoryAlreadyConnected,
}

impl axum::response::IntoResponse for ConnectError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::InvalidAuthorizationHeader(_) => StatusCode::BAD_REQUEST,
            Self::AccessoryNotFound => StatusCode::UNAUTHORIZED,
            Self::AccessoryAlreadyConnected => StatusCode::NOT_ACCEPTABLE,
        };
        let mut response = axum::Json(self).into_response();
        *response.status_mut() = status;

        response
    }
}

#[async_trait]
impl axum::extract::FromRequest<Body> for DeviceCredentials {
    type Rejection = ConnectError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(headers::Authorization(authorization)) =
            TypedHeader::<headers::Authorization<headers::authorization::Basic>>::from_request(req)
                .await
                .map_err(|err| ConnectError::InvalidAuthorizationHeader(err.to_string()))?;

        let accessory_id = accessory::ID::parse_str(authorization.username()).map_err(|err| {
            ConnectError::InvalidAuthorizationHeader(format!("invalid hub id: {}", err))
        })?;

        Ok(Self(accessory_id, authorization.password().to_owned()))
    }
}

pub async fn websocket_handler(
    websocket: ezsockets::axum::Upgrade,
    Extension(provider): Extension<providers::MasterHandle>,
    Extension(configured_accessories): Extension<ConfiguredAccessories>,
    Extension(server): Extension<Server>,
    DeviceCredentials(accessory_id, _password): DeviceCredentials,
) -> Result<impl axum::response::IntoResponse, ConnectError> {
    let accessory = configured_accessories
        .load()
        .iter()
        .find(|accessory| accessory.id == accessory_id)
        .ok_or(ConnectError::AccessoryNotFound)?
        .clone();
    let is_connected = provider.is_connected(accessory_id).await;
    if is_connected {
        return Err(ConnectError::AccessoryAlreadyConnected);
    }

    // TODO: Verify password and remove following line
    tracing::warn!("{} connected without password verification", accessory_id);

    Ok(websocket.on_upgrade(server, Args { accessory }))
}

pub struct HiveSession {
    session: Session,
    accessory_id: accessory::ID,
    controller: controllers::MasterHandle,
    characteristic_write_results:
        HashMap<hive::FrameID, oneshot::Sender<Result<(), accessory::Error>>>,
    characteristic_read_results: HashMap<
        hive::FrameID,
        oneshot::Sender<Result<accessory::characteristics::Characteristic, accessory::Error>>,
    >,
}

#[async_trait]
impl ezsockets::SessionExt for HiveSession {
    type ID = accessory::ID;
    type Params = SessionMessage;
    type Args = Args;

    fn id(&self) -> &Self::ID {
        &self.accessory_id
    }

    async fn text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let json = serde_json::from_str::<hive::AccessoryFrame>(&text)?;
        match json {
            hive::AccessoryFrame::UpdateCharacteristic(frame) => {
                self.controller
                    .updated(self.accessory_id, frame.service_name, frame.characteristic)
                    .await;
            }
            hive::AccessoryFrame::CharacteristicReadResult(frame) => self
                .characteristic_read_results
                .remove(&frame.id)
                .unwrap()
                .send(frame.result.into())
                .unwrap(),
            hive::AccessoryFrame::CharacteristicWriteResult(frame) => self
                .characteristic_write_results
                .remove(&frame.id)
                .unwrap()
                .send(frame.result.into())
                .unwrap(),
        }
        Ok(())
    }

    async fn binary(&mut self, _bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        todo!()
    }

    async fn call(&mut self, params: Self::Params) -> Result<(), ezsockets::Error> {
        match params {
            SessionMessage::ReadCharacteristic {
                service_name,
                characteristic_name,
                respond_to,
            } => {
                let frame_id = rand::random();
                let frame = hive::HubFrame::ReadCharacteristic(hive::ReadCharacteristic {
                    id: frame_id,
                    service_name,
                    characteristic_name,
                });
                let text = serde_json::to_string(&frame)?;
                let (response_tx, response_rx) = oneshot::channel();
                self.characteristic_read_results
                    .insert(frame_id, response_tx);
                self.session.text(text).await;
                respond_to.send(response_rx).unwrap();
            }
            SessionMessage::WriteCharacteristic {
                service_name,
                characteristic,
                respond_to,
            } => {
                let frame_id = rand::random();
                let frame = hive::HubFrame::WriteCharacteristic(hive::WriteCharacteristic {
                    id: frame_id,
                    service_name,
                    characteristic,
                });
                let text = serde_json::to_string(&frame)?;
                let (response_tx, response_rx) = oneshot::channel();
                self.characteristic_write_results
                    .insert(frame_id, response_tx);
                self.session.text(text).await;
                respond_to.send(response_rx).unwrap();
            }
        };
        Ok(())
    }
}
