use super::Handle;
use super::Message;
use super::Name;
use crate::controllers;
use anyhow::Context;
use anyhow::Error;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::ws;
use axum::extract::ws::WebSocket;
use axum::extract::Extension;
use axum::extract::TypedHeader;
use axum::headers;
use axum::http::StatusCode;
use axum::response::Response;
use axum::Router;
use futures::StreamExt;
use houseflow_config::server::providers::Lighthouse as Config;
use houseflow_config::server::providers::LighthouseHub;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::hub;
use houseflow_types::lighthouse;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum LighthouseMessage {
    Connected {
        hub: LighthouseHub,
        websocket_stream: WebSocket,
        respond_to: oneshot::Sender<LighthouseHubSessionHandle>,
    },
    Disconnected {
        hub_id: hub::ID,
    },
    GetHubConfiguration {
        hub_id: hub::ID,
        respond_to: oneshot::Sender<Option<LighthouseHub>>,
    },
}

#[derive(Debug, Clone)]
pub struct LighthouseHandle {
    sender: acu::Sender<LighthouseMessage>,
    handle: Handle,
}

impl std::ops::Deref for LighthouseHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl From<LighthouseHandle> for Handle {
    fn from(val: LighthouseHandle) -> Self {
        val.handle
    }
}

impl LighthouseHandle {
    pub async fn connected(
        &self,
        hub: LighthouseHub,
        websocket_stream: WebSocket,
    ) -> LighthouseHubSessionHandle {
        self.sender
            .call(|respond_to| LighthouseMessage::Connected {
                hub,
                websocket_stream,
                respond_to,
            })
            .await
    }

    pub async fn disconnected(&self, hub_id: hub::ID) {
        self.sender
            .notify(|| LighthouseMessage::Disconnected { hub_id })
            .await
    }

    pub async fn get_hub_configuration(&self, hub_id: hub::ID) -> Option<LighthouseHub> {
        self.sender
            .call(|respond_to| LighthouseMessage::GetHubConfiguration { hub_id, respond_to })
            .await
    }
}

pub struct LighthouseProvider {
    provider_receiver: acu::Receiver<Message>,
    lighthouse_receiver: acu::Receiver<LighthouseMessage>,
    controller: controllers::Handle,
    sessions: HashMap<hub::ID, LighthouseHubSessionHandle>,
    config: Config,
}

impl LighthouseProvider {
    pub fn create(controller: controllers::Handle, config: Config) -> LighthouseHandle {
        let (provider_sender, provider_receiver) = acu::channel(8, Name::Lighthouse.into());
        let (lighthouse_sender, lighthouse_receiver) = acu::channel(8, Name::Lighthouse.into());
        let mut actor = Self {
            provider_receiver,
            lighthouse_receiver,
            controller,
            sessions: Default::default(),
            config,
        };

        let handle = Handle::new(Name::Lighthouse, provider_sender);
        let handle = LighthouseHandle {
            sender: lighthouse_sender,
            handle,
        };
        tokio::spawn(async move { actor.run().await });
        handle
    }

    async fn run(&mut self) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(message) = self.provider_receiver.recv() => {
                    self.handle_provider_message(message).await?;
                },
                Some(message) = self.lighthouse_receiver.recv() => {
                    self.handle_lighthouse_message(message).await?
                },
                else => break,
            }
        }
        Ok(())
    }

    async fn handle_lighthouse_message(
        &mut self,
        message: LighthouseMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            LighthouseMessage::Connected {
                hub,
                websocket_stream,
                respond_to,
            } => {
                let session =
                    LighthouseHubSession::create(websocket_stream, self.controller.clone()).await;
                self.sessions.insert(hub.id, session.clone());
                respond_to.send(session).unwrap()
            }
            LighthouseMessage::Disconnected { hub_id } => {
                self.sessions.remove(&hub_id);
            }
            LighthouseMessage::GetHubConfiguration { hub_id, respond_to } => {
                let hub = self
                    .config
                    .hubs
                    .iter()
                    .find(|hub| hub.id == hub_id)
                    .cloned();
                respond_to.send(hub).unwrap();
            }
        };
        Ok(())
    }

    async fn find_accessory_session(
        &mut self,
        accessory_id: accessory::ID,
    ) -> Option<&LighthouseHubSessionHandle> {
        let values = self.sessions.iter().map(|(_, session)| async move {
            (session, session.is_accessory_connected(accessory_id).await)
        });
        let accessories = futures::future::join_all(values).await;
        accessories
            .into_iter()
            .find_map(|(session, is_connected)| if is_connected { Some(session) } else { None })
    }

    async fn handle_provider_message(&mut self, message: Message) -> Result<(), anyhow::Error> {
        match message {
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
                    .read_characteristic(accessory_id, service_name, characteristic_name)
                    .await;
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
                    .write_characteristic(accessory_id, service_name, characteristic)
                    .await;
                respond_to.send(result).unwrap();
            }
            Message::GetAccessories { respond_to } => {
                let accessories = self
                    .sessions
                    .values()
                    .map(|session| session.get_accessories());
                let accessories = futures::future::join_all(accessories).await;
                let accessories = accessories.into_iter().flatten().collect();
                respond_to.send(accessories).unwrap();
            }
            Message::IsConnected {
                accessory_id,
                respond_to,
            } => {
                let values = self
                    .sessions
                    .iter()
                    .map(|(_, session)| session.is_accessory_connected(accessory_id));
                let values = futures::future::join_all(values).await;
                let is_connected = values.iter().any(|is_connected| *is_connected);
                respond_to.send(is_connected).unwrap();
            }
        };
        Ok(())
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
    websocket: axum::extract::ws::WebSocketUpgrade,
    Extension(provider): Extension<LighthouseHandle>,
    HubCredentials(hub_id, _password): HubCredentials,
) -> Result<impl axum::response::IntoResponse, ConnectError> {
    let hub = provider
        .get_hub_configuration(hub_id)
        .await
        .ok_or(ConnectError::HubNotFound)?;
    let is_connected = provider.is_connected(hub_id).await;
    if is_connected {
        return Err(ConnectError::HubAlreadyConnected);
    }

    // TODO: Verify password and remove following line
    tracing::warn!("{} connected without password", hub_id);

    Ok(websocket.on_upgrade(move |stream| async move {
        let hub_id = hub.id;
        let session = provider.connected(hub, stream).await;
        session.wait_for_stop().await;
        provider.disconnected(hub_id).await;
    }))
}

pub fn app() -> Router {
    use axum::routing::get;

    Router::new().route("/websocket", get(websocket_handler))
}

#[derive(Debug)]
enum LighthouseHubSessionMessage {
    GetAccessories {
        respond_to: oneshot::Sender<Vec<accessory::ID>>,
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

pub struct LighthouseHubSession {
    lighthouse_hub_session_receiver: acu::Receiver<LighthouseHubSessionMessage>,
    controller: controllers::Handle,
    connected_accessories: HashSet<accessory::ID>,
    websocket_stream: WebSocket,
    characteristic_write_results:
        HashMap<lighthouse::FrameID, oneshot::Sender<Result<(), accessory::Error>>>,
    characteristic_read_results: HashMap<
        lighthouse::FrameID,
        oneshot::Sender<Result<accessory::characteristics::Characteristic, accessory::Error>>,
    >,
}

#[derive(Debug, Clone)]
pub struct LighthouseHubSessionHandle {
    sender: acu::Sender<LighthouseHubSessionMessage>,
}

impl LighthouseHubSessionHandle {
    pub async fn wait_for_stop(&self) {
        self.sender.closed().await
    }

    pub async fn read_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, accessory::Error> {
        self.sender
            .call(
                |respond_to| LighthouseHubSessionMessage::ReadCharacteristic {
                    accessory_id,
                    service_name,
                    characteristic_name,
                    respond_to,
                },
            )
            .await
            .await
            .unwrap()
    }

    pub async fn write_characteristic(
        &self,
        accessory_id: accessory::ID,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), accessory::Error> {
        self.sender
            .call(
                |respond_to| LighthouseHubSessionMessage::WriteCharacteristic {
                    accessory_id,
                    service_name,
                    characteristic,
                    respond_to,
                },
            )
            .await
            .await
            .unwrap()
    }

    pub async fn get_accessories(&self) -> Vec<accessory::ID> {
        self.sender
            .call(|respond_to| LighthouseHubSessionMessage::GetAccessories { respond_to })
            .await
    }

    pub async fn is_accessory_connected(&self, accessory_id: accessory::ID) -> bool {
        self.sender
            .call(
                |respond_to| LighthouseHubSessionMessage::IsAccessoryConnected {
                    accessory_id,
                    respond_to,
                },
            )
            .await
    }
}

impl LighthouseHubSession {
    pub async fn create(
        websocket_stream: WebSocket,
        controller: controllers::Handle,
    ) -> LighthouseHubSessionHandle {
        let (lighthouse_hub_session_sender, lighthouse_hub_session_receiver) =
            acu::channel(8, "LighthouseHubSession");
        let mut actor = Self {
            lighthouse_hub_session_receiver,
            connected_accessories: HashSet::new(),
            websocket_stream,
            controller,
            characteristic_write_results: HashMap::new(),
            characteristic_read_results: HashMap::new(),
        };
        let handle = LighthouseHubSessionHandle {
            sender: lighthouse_hub_session_sender,
        };
        tokio::spawn(async move { actor.run().await });

        handle
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        loop {
            tokio::select! {
                Some(message) = self.lighthouse_hub_session_receiver.recv() => {
                    self.handle_lighthouse_hub_session_message(message).await?
                },
                Some(message) = self.websocket_stream.next() => {
                    tracing::debug!("websocket message {:?}", message);
                    let message = message?;
                    self.handle_websocket_message(message).await?;
                }
                else => break,
            }
        }
        Ok(())
    }

    async fn send_websocket(&mut self, frame: lighthouse::ServerFrame) {
        let text = serde_json::to_string(&frame)
            .context("serializing frame failed")
            .unwrap();
        self.websocket_stream
            .send(ws::Message::Text(text))
            .await
            .unwrap();
    }

    async fn handle_websocket_message(
        &mut self,
        message: ws::Message,
    ) -> Result<(), anyhow::Error> {
        match message {
            ws::Message::Text(text) => {
                let frame = serde_json::from_str::<lighthouse::HubFrame>(&text)?;
                match frame {
                    lighthouse::HubFrame::AccessoryConnected(accessory) => {
                        self.connected_accessories.insert(accessory.id);
                        self.controller.connected(accessory).await;
                    }
                    lighthouse::HubFrame::AccessoryDisconnected(accessory_id) => {
                        self.connected_accessories.remove(&accessory_id);
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
            }
            ws::Message::Binary(_) => todo!(),
            ws::Message::Ping(_) => todo!(),
            ws::Message::Pong(_) => todo!(),
            ws::Message::Close(_) => todo!(),
        }
        Ok(())
    }

    async fn handle_lighthouse_hub_session_message(
        &mut self,
        message: LighthouseHubSessionMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            LighthouseHubSessionMessage::GetAccessories { respond_to } => respond_to
                .send(
                    self.connected_accessories
                        .clone()
                        .into_iter()
                        .collect::<Vec<_>>(),
                )
                .unwrap(),
            LighthouseHubSessionMessage::IsAccessoryConnected {
                accessory_id,
                respond_to,
            } => respond_to
                .send(self.connected_accessories.contains(&accessory_id))
                .unwrap(),
            LighthouseHubSessionMessage::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            } => {
                let id = rand::random();
                let frame = lighthouse::ReadCharacteristic {
                    id,
                    accessory_id,
                    service_name,
                    characteristic_name,
                };
                let (sender, receiver) = oneshot::channel();
                self.characteristic_read_results.insert(id, sender);
                self.send_websocket(lighthouse::ServerFrame::ReadCharacteristic(frame))
                    .await;
                respond_to.send(receiver).unwrap();
            }
            LighthouseHubSessionMessage::WriteCharacteristic {
                accessory_id,
                service_name,
                characteristic,
                respond_to,
            } => {
                let id = rand::random();
                let frame = lighthouse::WriteCharacteristic {
                    id,
                    accessory_id,
                    service_name,
                    characteristic,
                };
                let (sender, receiver) = oneshot::channel();
                self.characteristic_write_results.insert(id, sender);
                self.send_websocket(lighthouse::ServerFrame::WriteCharacteristic(frame))
                    .await;
                respond_to.send(receiver).unwrap();
            }
        };
        Ok(())
    }
}
