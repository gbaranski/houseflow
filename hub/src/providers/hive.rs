use super::ProviderMessage;
use super::SessionHandle;
use super::SessionMessage;
use crate::providers::ProviderName;
use crate::ControllerHandle;
use crate::ProviderHandle;
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
use houseflow_config::hub::Accessory;
use houseflow_config::hub::HiveProvider as Config;
use houseflow_types::accessory;
use houseflow_types::hive;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum HiveActorMessage {
    Connected {
        accessory: Accessory,
        session_handle: SessionHandle,
    },
    Disconnected {
        accessory_id: accessory::ID,
    },
}

#[derive(Debug, Clone)]
pub struct HiveProviderHandle {
    sender: mpsc::Sender<HiveActorMessage>,
    handle: ProviderHandle,
}

impl std::ops::Deref for HiveProviderHandle {
    type Target = ProviderHandle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl Into<ProviderHandle> for HiveProviderHandle {
    fn into(self) -> ProviderHandle {
        self.handle
    }
}

impl HiveProviderHandle {
    pub async fn connected(&self, accessory: Accessory, session_handle: SessionHandle) {
        self.sender
            .send(HiveActorMessage::Connected {
                accessory,
                session_handle,
            })
            .await
            .unwrap();
    }

    pub async fn disconnected(&self, accessory_id: accessory::ID) {
        self.sender
            .send(HiveActorMessage::Disconnected { accessory_id })
            .await
            .unwrap();
    }
}

pub struct HiveProvider {
    provider_receiver: mpsc::Receiver<ProviderMessage>,
    hive_receiver: mpsc::Receiver<HiveActorMessage>,
    controller: ControllerHandle,
    sessions: HashMap<accessory::ID, SessionHandle>,
    configured_accessories: Vec<Accessory>,
}

impl HiveProvider {
    pub fn new(
        controller: ControllerHandle,
        _config: Config,
        configured_accessories: Vec<Accessory>,
    ) -> HiveProviderHandle {
        let (provider_sender, provider_receiver) = tokio::sync::mpsc::channel(8);
        let (hive_sender, hive_receiver) = tokio::sync::mpsc::channel(8);
        let mut actor = Self {
            provider_receiver,
            hive_receiver,
            controller,
            configured_accessories,
            sessions: Default::default(),
        };

        let handle = ProviderHandle::new(ProviderName::Hive, provider_sender);
        let handle = HiveProviderHandle {
            sender: hive_sender,
            handle,
        };
        let app = app(actor.controller.clone(), handle.clone());
        tokio::spawn(async move { actor.run().await });
        tokio::spawn(async move {
            use std::net::Ipv4Addr;
            use std::net::SocketAddr;
            use std::net::SocketAddrV4;

            let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080));
            axum::Server::bind(&address)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
        handle
    }

    async fn run(&mut self) -> Result<(), Error> {
        loop {
            tokio::select! {
                Some(message) = self.provider_receiver.recv() => {
                    self.handle_provider_message(message).await?;
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
        message: HiveActorMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            HiveActorMessage::Connected {
                accessory,
                session_handle,
            } => {
                self.sessions.insert(accessory.id, session_handle);
                self.controller.connected(accessory).await;
            }
            HiveActorMessage::Disconnected { accessory_id } => {
                self.sessions.remove(&accessory_id);
                self.controller.disconnected(accessory_id).await;
            }
        };
        Ok(())
    }

    async fn handle_provider_message(
        &mut self,
        message: ProviderMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            ProviderMessage::ReadCharacteristic {
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
            ProviderMessage::WriteCharacteristic {
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
            ProviderMessage::GetAccessoryConfiguration {
                accessory_id,
                respond_to,
            } => {
                let accessory_configuration = self
                    .configured_accessories
                    .iter()
                    .find(|accessory| accessory.id == accessory_id)
                    .cloned();
                respond_to.send(accessory_configuration).unwrap();
            }
            ProviderMessage::IsConnected {
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

pub fn app(controller: ControllerHandle, hive_provider: HiveProviderHandle) -> axum::Router {
    use axum::routing::get;

    axum::Router::new()
        .route("/websocket", get(websocket_handler))
        .layer(axum::AddExtensionLayer::new(controller))
        .layer(axum::AddExtensionLayer::new(hive_provider))
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
    Extension(provider): Extension<HiveProviderHandle>,
    Extension(controller): Extension<ControllerHandle>,
    DeviceCredentials(accessory_id, _password): DeviceCredentials,
) -> Result<impl axum::response::IntoResponse, ConnectError> {
    let accessory = provider
        .get_accessory_configuration(accessory_id)
        .await
        .ok_or(ConnectError::AccessoryNotFound)?;
    let is_connected = provider.is_connected(accessory_id).await;
    if is_connected {
        return Err(ConnectError::AccessoryAlreadyConnected);
    }

    // TODO: Verify password and remove following line
    tracing::warn!("{} connected without password", accessory_id);

    Ok(websocket.on_upgrade(move |stream| async move {
        let session = HiveSession::new(accessory_id, stream, controller).await;
        let accessory_id = accessory.id;
        provider.connected(accessory, session.clone().into()).await;
        session.wait_for_stop().await;
        provider.disconnected(accessory_id).await;
    }))
}

#[derive(Debug)]
enum HiveSessionMessage {
    WebSocketMessage(ws::Message),
}

pub struct HiveSession {
    session_receiver: mpsc::Receiver<SessionMessage>,
    hive_receiver: mpsc::Receiver<HiveSessionMessage>,
    accessory_id: accessory::ID,
    controller: ControllerHandle,
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
    sender: mpsc::Sender<HiveSessionMessage>,
    handle: SessionHandle,
}

impl HiveSessionHandle {
    pub async fn websocket_message(&self, websocket_message: ws::Message) {
        self.sender
            .send(HiveSessionMessage::WebSocketMessage(websocket_message))
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

impl Into<SessionHandle> for HiveSessionHandle {
    fn into(self) -> SessionHandle {
        self.handle
    }
}

impl HiveSession {
    pub async fn new(
        accessory_id: accessory::ID,
        stream: WebSocket,
        controller: ControllerHandle,
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
        message: HiveSessionMessage,
    ) -> Result<(), anyhow::Error> {
        match message {
            HiveSessionMessage::WebSocketMessage(message) => {
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
