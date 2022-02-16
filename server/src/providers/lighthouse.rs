use super::Handle;
use super::Message;
use super::Name;
use crate::controllers;
use crate::controllers::ControllerExt;
use crate::providers::ProviderExt;
use anyhow::Context;
use anyhow::Error;
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::ws;
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
use std::fmt::Formatter;
use tokio::sync::oneshot;

// axum::extract::ws::WebSocket has ugly Debug log
pub struct WebSocket(ws::WebSocket);

impl std::ops::Deref for WebSocket {
    type Target = ws::WebSocket;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for WebSocket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for WebSocket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("WebSocket")
    }
}

#[derive(Debug)]
pub enum LighthouseProviderMessage {
    Connected {
        hub: LighthouseHub,
        websocket_stream: Box<WebSocket>,
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
impl acu::Message for LighthouseProviderMessage {}

type LighthouseProviderReceiver = acu::Receiver<LighthouseProviderMessage, Name>;

#[derive(Debug, Clone)]
pub struct LighthouseProviderHandle {
    handle: Handle,
    lighthouse: acu::Handle<LighthouseProviderMessage, Name>,
}

pub fn new(controller: controllers::MasterHandle, config: Config) -> LighthouseProviderHandle {
    let (sender, receiver) = acu::channel(8, Name::Lighthouse);
    let (lighthouse_sender, lighthouse_receiver) = acu::channel(8, Name::Lighthouse);
    let mut actor = LighthouseProvider {
        receiver,
        lighthouse_receiver,
        controller,
        sessions: Default::default(),
        config,
    };

    let lighthouse_handle = LighthouseProviderHandle {
        handle: Handle { sender },
        lighthouse: acu::Handle {
            sender: lighthouse_sender,
        },
    };
    tokio::spawn(async move { actor.run().await });
    lighthouse_handle
}

#[async_trait]
pub trait LighthouseProviderExt {
    async fn connected(
        &self,
        hub: LighthouseHub,
        websocket_stream: WebSocket,
    ) -> LighthouseHubSessionHandle;
    async fn disconnected(&self, hub_id: hub::ID);
    async fn get_hub_configuration(&self, hub_id: hub::ID) -> Option<LighthouseHub>;
}

#[async_trait]
impl LighthouseProviderExt for LighthouseProviderHandle {
    async fn connected(
        &self,
        hub: LighthouseHub,
        websocket_stream: WebSocket,
    ) -> LighthouseHubSessionHandle {
        self.lighthouse
            .sender
            .call_with(|respond_to| LighthouseProviderMessage::Connected {
                hub,
                websocket_stream: Box::new(websocket_stream),
                respond_to,
            })
            .await
    }

    async fn disconnected(&self, hub_id: hub::ID) {
        self.lighthouse
            .sender
            .notify(LighthouseProviderMessage::Disconnected { hub_id })
            .await
    }

    async fn get_hub_configuration(&self, hub_id: hub::ID) -> Option<LighthouseHub> {
        self.lighthouse
            .sender
            .call_with(
                |respond_to| LighthouseProviderMessage::GetHubConfiguration { hub_id, respond_to },
            )
            .await
    }
}

impl From<LighthouseProviderHandle> for Handle {
    fn from(handle: LighthouseProviderHandle) -> Self {
        handle.handle
    }
}
impl std::ops::Deref for LighthouseProviderHandle {
    type Target = Handle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

pub struct LighthouseProvider {
    receiver: acu::Receiver<Message, Name>,
    lighthouse_receiver: LighthouseProviderReceiver,
    controller: controllers::MasterHandle,
    sessions: HashMap<hub::ID, LighthouseHubSessionHandle>,
    config: Config,
}

impl LighthouseProvider {
    async fn run(&mut self) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(message) = self.receiver.recv() => {
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
        message: LighthouseProviderMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            LighthouseProviderMessage::Connected {
                hub,
                websocket_stream,
                respond_to,
            } => {
                let session =
                    LighthouseHubSession::create(*websocket_stream, self.controller.clone()).await;
                self.sessions.insert(hub.id, session.clone());
                respond_to.send(session).unwrap()
            }
            LighthouseProviderMessage::Disconnected { hub_id } => {
                self.sessions.remove(&hub_id);
            }
            LighthouseProviderMessage::GetHubConfiguration { hub_id, respond_to } => {
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
    Extension(provider): Extension<LighthouseProviderHandle>,
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
        let session = provider.connected(hub, WebSocket(stream)).await;
        session.wait_for_stop().await;
        provider.disconnected(hub_id).await;
    }))
}

pub fn app(handle: LighthouseProviderHandle) -> Router {
    use axum::routing::get;

    Router::new()
        .route("/websocket", get(websocket_handler))
        .layer(axum::AddExtensionLayer::new(handle))
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

impl acu::Message for LighthouseHubSessionMessage {}

type LighthouseHubSessionReceiver = acu::Receiver<LighthouseHubSessionMessage, &'static str>;
type LighthouseHubSessionSender = acu::Sender<LighthouseHubSessionMessage, &'static str>;

pub struct LighthouseHubSession {
    lighthouse_hub_session_receiver: LighthouseHubSessionReceiver,
    controller: controllers::MasterHandle,
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
    sender: LighthouseHubSessionSender,
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
            .call_with(
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
            .call_with(
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
            .call_with(|respond_to| LighthouseHubSessionMessage::GetAccessories { respond_to })
            .await
    }

    pub async fn is_accessory_connected(&self, accessory_id: accessory::ID) -> bool {
        self.sender
            .call_with(
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
        controller: controllers::MasterHandle,
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
