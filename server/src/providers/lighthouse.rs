use super::Handle;
use super::Message;
use super::Name;
use super::SessionHandle;
use super::SessionMessage;
use crate::controllers;
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
use futures::stream::SplitSink;
use futures::stream::SplitStream;
use futures::SinkExt;
use futures::StreamExt;
use houseflow_config::server::Config;
use houseflow_config::server::Hub;
use houseflow_types::accessory;
use houseflow_types::accessory::Accessory;
use houseflow_types::hive;
use houseflow_types::hub;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum LighthouseMessage {
    Connected {
        hub: Hub,
        session_handle: SessionHandle,
    },
    Disconnected {
        hub_id: hub::ID,
    },
    AccessoryConnected {
        accessory: Accessory,
    },
    AccessoryDisconnected {
        accessory_id: accessory::ID,
    },
    GetHubConfiguration {
        hub_id: hub::ID,
        respond_to: oneshot::Sender<Option<Hub>>,
    },
}

#[derive(Debug, Clone)]
pub struct LighthouseHandle {
    sender: mpsc::Sender<LighthouseMessage>,
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
    pub async fn connected(&self, hub: Hub, session_handle: SessionHandle) {
        self.notify_lighthouse(|| LighthouseMessage::Connected {
            hub,
            session_handle,
        })
        .await
    }

    pub async fn disconnected(&self, hub_id: hub::ID) {
        self.notify_lighthouse(|| LighthouseMessage::Disconnected { hub_id })
            .await
    }

    pub async fn get_hub_configuration(&self, hub_id: hub::ID) -> Option<Hub> {
        self.call_lighthouse(|respond_to| LighthouseMessage::GetHubConfiguration {
            hub_id,
            respond_to,
        })
        .await
    }
}

impl LighthouseHandle {
    async fn notify_lighthouse(&self, message_fn: impl FnOnce() -> LighthouseMessage) {
        let message = message_fn();
        tracing::debug!("notifying {:?} on a lighthouse controller", message);
        self.sender.send(message).await.unwrap();
    }

    async fn call_lighthouse<R>(
        &self,
        message_fn: impl FnOnce(oneshot::Sender<R>) -> LighthouseMessage,
    ) -> R {
        let (tx, rx) = oneshot::channel();
        let message = message_fn(tx);
        tracing::debug!("calling {:?} on a lighthouse controller", message);
        self.sender.send(message).await.unwrap();
        rx.await.unwrap()
    }
}

pub struct LighthouseProvider {
    provider_receiver: mpsc::Receiver<Message>,
    lighthouse_receiver: mpsc::Receiver<LighthouseMessage>,
    controller: controllers::Handle,
    sessions: HashMap<accessory::ID, SessionHandle>,
    connected_accessories: Vec<Accessory>,
    configured_hubs: Vec<Hub>,
}

impl LighthouseProvider {
    pub fn create(
        controller: controllers::Handle,
        _config: Config,
        configured_hubs: Vec<Hub>,
    ) -> LighthouseHandle {
        let (provider_sender, provider_receiver) = tokio::sync::mpsc::channel(8);
        let (lighthouse_sender, lighthouse_receiver) = tokio::sync::mpsc::channel(8);
        let mut actor = Self {
            provider_receiver,
            lighthouse_receiver,
            controller,
            connected_accessories: vec![],
            configured_hubs,
            sessions: Default::default(),
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
                    self.handle_hive_message(message).await?
                },
                else => break,
            }
        }
        Ok(())
    }

    async fn handle_hive_message(
        &mut self,
        message: LighthouseMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            LighthouseMessage::Connected {
                hub,
                session_handle,
            } => {
                self.sessions.insert(hub.id, session_handle);
            }
            LighthouseMessage::Disconnected { hub_id } => {
                self.sessions.remove(&hub_id);
            }
            LighthouseMessage::GetHubConfiguration { hub_id, respond_to } => {
                let hub = self.configured_hubs.iter().find(|hub| hub.id == hub_id).cloned();
                respond_to.send(hub).unwrap();
            }
            LighthouseMessage::AccessoryConnected { accessory } => {
                self.connected_accessories.push(accessory.clone());
                self.controller.connected(accessory).await;
            }
            LighthouseMessage::AccessoryDisconnected { accessory_id } => {
                self.connected_accessories
                    .retain(|accessory| accessory.id != accessory_id);
                self.controller.disconnected(accessory_id).await;
            }
        };
        Ok(())
    }

    async fn handle_provider_message(&mut self, message: Message) -> Result<(), anyhow::Error> {
        match message {
            Message::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            } => {
                let session = self.sessions.get(&accessory_id).unwrap();
                let result = session
                    .read_characteristic(service_name, characteristic_name)
                    .await;
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
                    .write_characteristic(service_name, characteristic)
                    .await;
                respond_to.send(result).unwrap();
            }
            Message::GetAccessories { respond_to } => {
                let accessories = self
                    .connected_accessories
                    .iter()
                    .map(|accessory| accessory.id)
                    .collect();
                respond_to.send(accessories).unwrap();
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
        tracing::info!("hello world 1");

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
    websocket: axum::extract::ws::WebSocketUpgrade,
    Extension(provider): Extension<LighthouseHandle>,
    Extension(controller): Extension<controllers::Handle>,
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
        let session = LighthouseSession::create(hub_id, stream, controller).await;
        let hub_id = hub.id;
        provider.connected(hub, session.clone().into()).await;
        session.wait_for_stop().await;
        provider.disconnected(hub_id).await;
    }))
}

#[derive(Debug)]
enum LighthouseSessionMessage {
    WebSocketMessage(ws::Message),
}

pub struct LighthouseSession {
    session_receiver: mpsc::Receiver<SessionMessage>,
    hive_receiver: mpsc::Receiver<LighthouseSessionMessage>,
    accessory_id: accessory::ID,
    controller: controllers::Handle,
    characteristic_write_results:
        HashMap<hive::FrameID, oneshot::Sender<Result<(), accessory::Error>>>,
    characteristic_read_results: HashMap<
        hive::FrameID,
        oneshot::Sender<Result<accessory::characteristics::Characteristic, accessory::Error>>,
    >,
    sink: SplitSink<WebSocket, ws::Message>,
}

#[derive(Debug, Clone)]
pub struct HiveSessionHandle {
    sender: mpsc::Sender<LighthouseSessionMessage>,
    handle: SessionHandle,
}

impl HiveSessionHandle {
    pub async fn websocket_message(&self, websocket_message: ws::Message) {
        self.sender
            .send(LighthouseSessionMessage::WebSocketMessage(
                websocket_message,
            ))
            .await
            .unwrap();
    }
}

impl std::ops::Deref for HiveSessionHandle {
    type Target = SessionHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl From<HiveSessionHandle> for SessionHandle {
    fn from(val: HiveSessionHandle) -> Self {
        val.handle
    }
}

impl LighthouseSession {
    pub async fn create(
        accessory_id: accessory::ID,
        stream: WebSocket,
        controller: controllers::Handle,
    ) -> HiveSessionHandle {
        let (session_sender, session_receiver) = mpsc::channel(8);
        let (hive_sender, hive_receiver) = mpsc::channel(8);
        let (sink, stream) = stream.split();
        let mut actor = Self {
            hive_receiver,
            session_receiver,
            accessory_id,
            characteristic_write_results: Default::default(),
            characteristic_read_results: Default::default(),
            sink,
            controller,
        };
        let handle = SessionHandle::new(session_sender);
        let handle = HiveSessionHandle {
            sender: hive_sender,
            handle,
        };
        {
            let handle = handle.clone();
            tokio::spawn(async move { actor.run(handle, stream).await });
        }

        handle
    }

    async fn run(
        &mut self,
        handle: HiveSessionHandle,
        mut stream: SplitStream<WebSocket>,
    ) -> Result<(), anyhow::Error> {
        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                let message = message?;
                handle.websocket_message(message).await;
            }
            Ok::<_, anyhow::Error>(())
        });
        loop {
            tokio::select! {
                Some(message) = self.session_receiver.recv() => {
                    self.handle_session_message(message).await?;
                },
                Some(message) = self.hive_receiver.recv() => {
                    self.handle_hive_message(message).await?
                },
                else => break,
            }
        }
        Ok(())
    }

    async fn handle_hive_message(
        &mut self,
        message: LighthouseSessionMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            LighthouseSessionMessage::WebSocketMessage(message) => {
                match message {
                    ws::Message::Text(s) => {
                        let json = serde_json::from_str::<hive::AccessoryFrame>(&s)?;
                        match json {
                            hive::AccessoryFrame::CharacteristicUpdate(frame) => {
                                self.controller
                                    .updated(
                                        self.accessory_id,
                                        frame.service_name,
                                        frame.characteristic,
                                    )
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
                    }
                    ws::Message::Binary(_) => todo!(),
                    ws::Message::Ping(bytes) => {
                        self.sink.send(ws::Message::Pong(bytes)).await?;
                    }
                    ws::Message::Pong(bytes) => {
                        tracing::info!("pong: {:?}", bytes);
                    }
                    ws::Message::Close(_) => todo!(),
                };
            }
        };
        Ok(())
    }

    async fn handle_session_message(
        &mut self,
        message: SessionMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            SessionMessage::ReadCharacteristic {
                service_name,
                characteristic_name,
                respond_to,
            } => {
                let frame_id = rand::random();
                let frame = hive::HubFrame::CharacteristicRead(hive::CharacteristicRead {
                    id: frame_id,
                    service_name,
                    characteristic_name,
                });
                let text = serde_json::to_string(&frame)?;
                let message = ws::Message::Text(text);
                let (response_tx, response_rx) = oneshot::channel();
                self.characteristic_read_results
                    .insert(frame_id, response_tx);
                self.sink.send(message).await?;
                respond_to.send(response_rx).unwrap();
            }
            SessionMessage::WriteCharacteristic {
                service_name,
                characteristic,
                respond_to,
            } => {
                let frame_id = rand::random();
                let frame = hive::HubFrame::CharacteristicWrite(hive::CharacteristicWrite {
                    id: frame_id,
                    service_name,
                    characteristic,
                });
                let text = serde_json::to_string(&frame)?;
                let message = ws::Message::Text(text);
                let (response_tx, response_rx) = oneshot::channel();
                self.characteristic_write_results
                    .insert(frame_id, response_tx);
                self.sink.send(message).await?;
                respond_to.send(response_rx).unwrap();
            }
        };
        Ok(())
    }
}
