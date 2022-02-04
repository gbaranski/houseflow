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
use axum::Router;
use futures::stream::SplitSink;
use futures::stream::SplitStream;
use futures::SinkExt;
use futures::StreamExt;
use houseflow_config::server::providers::Lighthouse as Config;
use houseflow_config::server::providers::LighthouseHub;
use houseflow_types::accessory;
use houseflow_types::accessory::Accessory;
use houseflow_types::hub;
use houseflow_types::lighthouse;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum LighthouseMessage {
    Connected {
        hub: LighthouseHub,
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
    pub async fn connected(&self, hub: LighthouseHub, session_handle: SessionHandle) {
        self.sender
            .notify(|| LighthouseMessage::Connected {
                hub,
                session_handle,
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
    sessions: HashMap<accessory::ID, SessionHandle>,
    connected_accessories: Vec<Accessory>,
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
            connected_accessories: vec![],
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
                session_handle,
            } => {
                self.sessions.insert(hub.id, session_handle);
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
        let TypedHeader(headers::Authorization(authorization)) =
            TypedHeader::<headers::Authorization<headers::authorization::Basic>>::from_request(req)
                .await
                .map_err(|err| ConnectError::InvalidAuthorizationHeader(err.to_string()))?;
        let accessory_id = accessory::ID::parse_str(authorization.username()).map_err(|err| {
            dbg!();
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
    dbg!();
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

pub fn app() -> Router {
    use axum::routing::get;

    Router::new().route("/websocket", get(websocket_handler))
}

#[derive(Debug)]
enum LighthouseSessionMessage {
    WebSocketMessage(ws::Message),
}

pub struct LighthouseSession {
    session_receiver: acu::Receiver<SessionMessage>,
    lighthouse_receiver: acu::Receiver<LighthouseSessionMessage>,
    accessory_id: accessory::ID,
    controller: controllers::Handle,
    characteristic_write_results:
        HashMap<lighthouse::FrameID, oneshot::Sender<Result<(), accessory::Error>>>,
    characteristic_read_results: HashMap<
        lighthouse::FrameID,
        oneshot::Sender<Result<accessory::characteristics::Characteristic, accessory::Error>>,
    >,
    sink: SplitSink<WebSocket, ws::Message>,
}

#[derive(Debug, Clone)]
pub struct LighthouseSessionHandle {
    sender: acu::Sender<LighthouseSessionMessage>,
    handle: SessionHandle,
}

impl LighthouseSessionHandle {
    pub async fn websocket_message(&self, websocket_message: ws::Message) {
        self.sender
            .notify(|| LighthouseSessionMessage::WebSocketMessage(websocket_message))
            .await;
    }
}

impl std::ops::Deref for LighthouseSessionHandle {
    type Target = SessionHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl From<LighthouseSessionHandle> for SessionHandle {
    fn from(val: LighthouseSessionHandle) -> Self {
        val.handle
    }
}

impl LighthouseSession {
    pub async fn create(
        accessory_id: accessory::ID,
        stream: WebSocket,
        controller: controllers::Handle,
    ) -> LighthouseSessionHandle {
        let (session_sender, session_receiver) = acu::channel(8, "LighthouseSession");
        let (lighthouse_sender, lighthouse_receiver) = acu::channel(8, "LighthouseSession");
        let (sink, stream) = stream.split();
        let mut actor = Self {
            lighthouse_receiver,
            session_receiver,
            accessory_id,
            characteristic_write_results: Default::default(),
            characteristic_read_results: Default::default(),
            sink,
            controller,
        };
        let handle = SessionHandle::new(session_sender);
        let handle = LighthouseSessionHandle {
            sender: lighthouse_sender,
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
        handle: LighthouseSessionHandle,
        mut stream: SplitStream<WebSocket>,
    ) -> Result<(), anyhow::Error> {
        let mut read_messages_future = tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                tracing::debug!("websocket message {:?}", message);
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
                Some(message) = self.lighthouse_receiver.recv() => {
                    self.handle_lighthouse_message(message).await?
                },
                result = &mut read_messages_future => {
                    let result = result.unwrap();
                    match result {
                        Ok(_) => {
                            tracing::info!("reading websocket messages finished");
                        },
                        Err(error) => {
                            tracing::error!("reading websocket messages failed due to `{}`", error);
                        },
                    }
                    break;
                }
                else => break,
            }
        }
        Ok(())
    }

    async fn handle_lighthouse_message(
        &mut self,
        message: LighthouseSessionMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            LighthouseSessionMessage::WebSocketMessage(message) => {
                match message {
                    ws::Message::Text(s) => {
                        let json = serde_json::from_str::<lighthouse::HubFrame>(&s)?;
                        match json {
                            lighthouse::HubFrame::CharacteristicUpdate(frame) => {
                                self.controller
                                    .updated(
                                        self.accessory_id,
                                        frame.service_name,
                                        frame.characteristic,
                                    )
                                    .await;
                            }
                            lighthouse::HubFrame::CharacteristicReadResult(frame) => self
                                .characteristic_read_results
                                .remove(&frame.id)
                                .unwrap()
                                .send(frame.result.into())
                                .unwrap(),
                            lighthouse::HubFrame::CharacteristicWriteResult(frame) => self
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
                let frame =
                    lighthouse::ServerFrame::CharacteristicRead(lighthouse::CharacteristicRead {
                        id: frame_id,
                        accessory_id: self.accessory_id,
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
                let frame =
                    lighthouse::ServerFrame::CharacteristicWrite(lighthouse::CharacteristicWrite {
                        id: frame_id,
                        accessory_id: self.accessory_id,
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
