use super::Message;
use crate::controllers;
use crate::controllers::ControllerExt;
use crate::ConfiguredHubs;
use anyhow::Context;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::Extension;
use axum::extract::TypedHeader;
use axum::headers;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Router;
use houseflow_config::server::providers::Lighthouse as Config;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::accessory::Accessory;
use houseflow_types::hub;
use houseflow_types::lighthouse;
use houseflow_types::structure;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::oneshot;

pub type Server = ezsockets::Server<LighthouseProvider>;
pub type Session = ezsockets::Session<accessory::ID, SessionMessage>;

#[derive(Debug)]
pub enum LighthouseProviderMessage {
    IsHubConnected {
        id: hub::ID,
        respond_to: oneshot::Sender<bool>,
    },
    Message(Message),
}

#[derive(Debug, Clone)]
pub struct LighthouseProvider {
    sessions: HashMap<hub::ID, Session>,
    controller: controllers::MasterHandle,
    config: Config,
}

pub fn new(master_controller: controllers::MasterHandle, config: Config) -> Server {
    let (server, _) = Server::create(|_| LighthouseProvider {
        sessions: Default::default(),
        controller: master_controller,
        config,
    });
    server
}

#[async_trait]
impl ezsockets::ServerExt for LighthouseProvider {
    type Session = LighthouseSession;

    type Params = LighthouseProviderMessage;

    async fn accept(
        &mut self,
        socket: ezsockets::Socket,
        _address: std::net::SocketAddr,
        (hub_id, hub_password_hash): <Self::Session as ezsockets::SessionExt>::Args,
    ) -> Result<
        ezsockets::Session<
            <Self::Session as ezsockets::SessionExt>::ID,
            <Self::Session as ezsockets::SessionExt>::Params,
        >,
        ezsockets::Error,
    > {
        let hub = self
            .config
            .hubs
            .iter()
            .find(|hub| hub.id == hub_id)
            .unwrap();
        assert_eq!(hub.password_hash, hub_password_hash);
        let session = Session::create(
            |handle| LighthouseSession {
                session: handle,
                hub_id,
                structure_id: hub.structure_id,
                controller: self.controller.clone(),
                connected_accessories: Default::default(),
                characteristic_write_results: Default::default(),
                characteristic_read_results: Default::default(),
            },
            hub_id,
            socket,
        );
        self.sessions.insert(hub_id, session.clone());
        Ok(session)
    }

    async fn disconnected(
        &mut self,
        id: <Self::Session as ezsockets::SessionExt>::ID,
    ) -> Result<(), ezsockets::Error> {
        self.sessions.remove(&id);
        Ok(())
    }

    async fn call(&mut self, params: Self::Params) -> Result<(), ezsockets::Error> {
        match params {
            LighthouseProviderMessage::IsHubConnected { id, respond_to } => {
                respond_to.send(self.sessions.contains_key(&id)).unwrap();
            }
            LighthouseProviderMessage::Message(message) => match message {
                Message::ReadCharacteristic {
                    accessory_id,
                    service_name,
                    characteristic_name,
                    respond_to,
                } => {
                    let hub_session = self
                        .find_accessory_session(accessory_id)
                        .await
                        .context("hub with the accessory is not connected")?;
                    let result = hub_session
                        .call_with(|respond_to| SessionMessage::ReadCharacteristic {
                            accessory_id,
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
                    let hub_session = self
                        .find_accessory_session(accessory_id)
                        .await
                        .context("hub with the accessory is not connected")?;
                    let result = hub_session
                        .call_with(|respond_to| SessionMessage::WriteCharacteristic {
                            accessory_id,
                            service_name,
                            characteristic,
                            respond_to,
                        })
                        .await
                        .await
                        .unwrap();
                    respond_to.send(result).unwrap();
                }
                Message::GetAccessories { respond_to } => {
                    let accessories = self.sessions.values().map(|session| async move {
                        (
                            session
                                .call_with(|respond_to| SessionMessage::GetStructureID {
                                    respond_to,
                                })
                                .await,
                            session
                                .call_with(|respond_to| SessionMessage::GetAccessories {
                                    respond_to,
                                })
                                .await,
                        )
                    });
                    let accessories = futures::future::join_all(accessories).await;
                    let accessories = accessories.into_iter().collect();
                    respond_to.send(accessories).unwrap();
                }
                Message::IsConnected {
                    accessory_id,
                    respond_to,
                } => {
                    let values = self.sessions.iter().map(|(_, session)| {
                        session.call_with(|respond_to| SessionMessage::IsAccessoryConnected {
                            accessory_id,
                            respond_to,
                        })
                    });
                    let values = futures::future::join_all(values).await;
                    let is_connected = values.iter().any(|is_connected| *is_connected);
                    respond_to.send(is_connected).unwrap();
                }
            },
        };
        Ok(())
    }
}

impl LighthouseProvider {
    async fn find_accessory_session(&mut self, accessory_id: accessory::ID) -> Option<&Session> {
        let values = self.sessions.iter().map(|(_, session)| async move {
            (
                session,
                session
                    .call_with(|respond_to| SessionMessage::IsAccessoryConnected {
                        accessory_id,
                        respond_to,
                    })
                    .await,
            )
        });
        let accessories = futures::future::join_all(values).await;
        accessories
            .into_iter()
            .find_map(|(session, is_connected)| if is_connected { Some(session) } else { None })
    }
}

pub struct HubCredentials(hub::ID, hub::Password);

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "error", content = "description")]
pub enum ConnectError {
    InvalidAuthorizationHeader(String),
    HubNotFound,
    HubAlreadyConnected,
}

impl axum::response::IntoResponse for ConnectError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::InvalidAuthorizationHeader(_) => StatusCode::BAD_REQUEST,
            Self::HubNotFound => StatusCode::UNAUTHORIZED,
            Self::HubAlreadyConnected => StatusCode::NOT_ACCEPTABLE,
        };
        let mut response = axum::Json(self).into_response();
        *response.status_mut() = status;

        response
    }
}

#[async_trait]
impl axum::extract::FromRequest<Body> for HubCredentials {
    type Rejection = axum::Json<ConnectError>;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(headers::Authorization(authorization)) =
            TypedHeader::<headers::Authorization<headers::authorization::Basic>>::from_request(req)
                .await
                .map_err(|err| ConnectError::InvalidAuthorizationHeader(err.to_string()))?;
        let hub_id = hub::ID::parse_str(authorization.username()).map_err(|err| {
            dbg!();
            ConnectError::InvalidAuthorizationHeader(format!("invalid hub id: {}", err))
        })?;

        Ok(Self(hub_id, authorization.password().to_owned()))
    }
}

pub async fn websocket_handler(
    websocket: ezsockets::axum::Upgrade,
    Extension(configured_hubs): Extension<ConfiguredHubs>,
    Extension(server): Extension<Server>,
    HubCredentials(hub_id, password): HubCredentials,
) -> Result<impl axum::response::IntoResponse, ConnectError> {
    let hub = configured_hubs
        .load()
        .iter()
        .find(|hub| hub.id == hub_id)
        .cloned()
        .ok_or(ConnectError::HubNotFound)?;
    let is_connected = server
        .call_with(|respond_to| LighthouseProviderMessage::IsHubConnected {
            id: hub.id,
            respond_to,
        })
        .await;
    if is_connected {
        return Err(ConnectError::HubAlreadyConnected);
    }

    // TODO: Verify password and remove following line
    tracing::warn!("{} connected without password", hub_id);
    Ok(websocket.on_upgrade(server, (hub_id, password)))
}

pub fn app(server: Server) -> Router {
    use axum::routing::get;

    Router::new()
        .route("/websocket", get(websocket_handler))
        .layer(Extension(server))
}

#[derive(Debug)]
pub enum SessionMessage {
    GetStructureID {
        respond_to: oneshot::Sender<structure::ID>,
    },
    GetAccessories {
        respond_to: oneshot::Sender<Vec<Accessory>>,
    },
    IsAccessoryConnected {
        accessory_id: accessory::ID,
        respond_to: oneshot::Sender<bool>,
    },
    ReadCharacteristic {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
        respond_to: oneshot::Sender<oneshot::Receiver<Result<Characteristic, accessory::Error>>>,
    },
    WriteCharacteristic {
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
        respond_to: oneshot::Sender<oneshot::Receiver<Result<(), accessory::Error>>>,
    },
}

pub struct LighthouseSession {
    session: ezsockets::Session<hub::ID, SessionMessage>,
    hub_id: hub::ID,
    structure_id: structure::ID,
    controller: controllers::MasterHandle,
    connected_accessories: HashMap<accessory::ID, Accessory>,
    characteristic_write_results:
        HashMap<lighthouse::FrameID, oneshot::Sender<Result<(), accessory::Error>>>,
    characteristic_read_results: HashMap<
        lighthouse::FrameID,
        oneshot::Sender<Result<accessory::characteristics::Characteristic, accessory::Error>>,
    >,
}

impl LighthouseSession {
    async fn send(&mut self, message: lighthouse::ServerFrame) -> Result<(), ezsockets::Error> {
        let json = serde_json::to_string(&message)?;
        self.session.text(json).await;
        Ok(())
    }
}

#[async_trait]
impl ezsockets::SessionExt for LighthouseSession {
    type ID = hub::ID;
    type Args = (hub::ID, hub::PasswordHash);
    type Params = SessionMessage;

    fn id(&self) -> &Self::ID {
        &self.hub_id
    }

    async fn text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let frame = serde_json::from_str::<lighthouse::HubFrame>(&text)?;
        match frame {
            lighthouse::HubFrame::AccessoryConnected(accessory) => {
                self.connected_accessories
                    .insert(accessory.id, accessory.clone());
                self.controller.connected(accessory).await;
            }
            lighthouse::HubFrame::AccessoryDisconnected(accessory_id) => {
                self.connected_accessories.remove(&accessory_id).unwrap();
                self.controller.disconnected(accessory_id).await;
            }
            lighthouse::HubFrame::UpdateCharacteristic(frame) => {
                self.controller
                    .updated(frame.accessory_id, frame.service_name, frame.characteristic)
                    .await;
            }
            lighthouse::HubFrame::ReadCharacteristicResult(frame) => {
                self.characteristic_read_results
                    .remove(&frame.id)
                    .unwrap()
                    .send(frame.result.into())
                    .unwrap();
            }
            lighthouse::HubFrame::WriteCharacteristicResult(frame) => {
                self.characteristic_write_results
                    .remove(&frame.id)
                    .unwrap()
                    .send(frame.result.into())
                    .unwrap();
            }
        };
        Ok(())
    }

    async fn binary(&mut self, _: Vec<u8>) -> Result<(), ezsockets::Error> {
        todo!()
    }

    async fn call(&mut self, params: Self::Params) -> Result<(), ezsockets::Error> {
        match params {
            SessionMessage::GetStructureID { respond_to } => {
                respond_to.send(self.structure_id).unwrap()
            },
            SessionMessage::GetAccessories { respond_to } => respond_to
                .send(
                    self.connected_accessories
                        .clone()
                        .into_values()
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            SessionMessage::IsAccessoryConnected {
                accessory_id,
                respond_to,
            } => respond_to
                .send(self.connected_accessories.get(&accessory_id).is_some())
                .unwrap(),
            SessionMessage::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            } => {
                let id = rand::random();
                let (sender, receiver) = oneshot::channel();
                self.characteristic_read_results.insert(id, sender);
                self.send(lighthouse::ServerFrame::ReadCharacteristic(
                    lighthouse::ReadCharacteristic {
                        id,
                        accessory_id,
                        service_name,
                        characteristic_name,
                    },
                ))
                .await?;
                respond_to.send(receiver).unwrap();
            }
            SessionMessage::WriteCharacteristic {
                accessory_id,
                service_name,
                characteristic,
                respond_to,
            } => {
                let id = rand::random();
                let (sender, receiver) = oneshot::channel();
                self.characteristic_write_results.insert(id, sender);
                self.send(lighthouse::ServerFrame::WriteCharacteristic(
                    lighthouse::WriteCharacteristic {
                        id,
                        accessory_id,
                        service_name,
                        characteristic,
                    },
                ))
                .await?;
                respond_to.send(receiver).unwrap();
            }
        };
        Ok(())
    }
}
