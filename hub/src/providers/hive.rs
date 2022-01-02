use super::Event;
use super::EventSender;
use super::Provider;
use anyhow::Error;
use async_trait::async_trait;
use futures::stream::SplitSink;
use futures::SinkExt;
use futures::StreamExt;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::HiveProvider as Config;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicDiscriminants;
use houseflow_types::accessory::services::ServiceDiscriminants;
use houseflow_types::hive;
use messages::prelude::*;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::oneshot;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::WebSocketStream;

pub struct HiveProvider {
    address: Address<HiveProviderActor>,
    tcp_listener: TcpListener,
    global_events: EventSender,
}

impl HiveProvider {
    pub async fn new(
        _config: Config,
        configured_accessories: Vec<Accessory>,
        global_events: EventSender,
    ) -> Result<Self, anyhow::Error> {
        let tcp_listener = TcpListener::bind("0.0.0.0:8080").await?;
        let actor = HiveProviderActor::new(_config, configured_accessories);
        let address = actor.spawn();
        Ok(Self {
            address,
            global_events,
            tcp_listener,
        })
    }
}

pub struct HiveProviderActor {
    configured_accessories: Vec<Accessory>,
    sessions: HashMap<accessory::ID, Address<SessionActor>>,
}

#[derive(Debug)]
struct Connected {
    sink: SplitSink<WebSocketStream<TcpStream>, tungstenite::Message>,
}

#[derive(Debug)]
struct Disconnected {
    accessory_id: accessory::ID,
}

#[derive(Debug)]
struct WriteCharacteristic {
    accessory_id: accessory::ID,
    service_name: accessory::services::ServiceDiscriminants,
    characteristic: accessory::characteristics::Characteristic,
}

#[derive(Debug)]
struct ReadCharacteristic {
    accessory_id: accessory::ID,
    service_name: accessory::services::ServiceDiscriminants,
    characteristic_name: accessory::characteristics::CharacteristicDiscriminants,
}

#[derive(Debug)]
struct IsConnected {
    accessory_id: accessory::ID,
}

#[derive(Debug)]
struct NewSession {
    accessory: Accessory,
    session: Address<SessionActor>,
}

impl HiveProviderActor {
    pub fn new(_config: Config, configured_accessories: Vec<Accessory>) -> Self {
        Self {
            configured_accessories,
            sessions: Default::default(),
        }
    }
}

#[async_trait]
impl Actor for HiveProviderActor {}

#[async_trait]
impl Handler<Connected> for HiveProviderActor {
    type Result = Result<NewSession, Error>;

    async fn handle(&mut self, input: Connected, _context: &Context<Self>) -> Self::Result {
        let accessory = self.configured_accessories.first().unwrap().to_owned(); // TODO: Parse credentials from HTTP Headers

        let session = {
            let actor = SessionActor::new(accessory.id, input.sink);
            let context = Context::new();
            let address = context.address();
            tokio::spawn(context.run(actor));
            address
        };
        self.sessions.insert(accessory.id, session.clone());

        Ok(NewSession { accessory, session })
    }
}

#[async_trait]
impl Handler<Disconnected> for HiveProviderActor {
    type Result = ();

    async fn handle(&mut self, input: Disconnected, _context: &Context<Self>) -> Self::Result {
        self.sessions.remove(&input.accessory_id);
    }
}

#[async_trait]
impl Handler<WriteCharacteristic> for HiveProviderActor {
    type Result = Result<Result<(), accessory::Error>, Error>;

    async fn handle(
        &mut self,
        input: WriteCharacteristic,
        _context: &Context<Self>,
    ) -> Self::Result {
        let oneshot = {
            let session = self.sessions.get_mut(&input.accessory_id).unwrap();
            session
                .send(SessionWriteCharacteristic {
                    service_name: input.service_name,
                    characteristic: input.characteristic,
                })
                .await??
        };
        Ok(oneshot.await?)
    }
}

#[async_trait]
impl Handler<ReadCharacteristic> for HiveProviderActor {
    type Result = Result<Result<Characteristic, accessory::Error>, Error>;

    async fn handle(
        &mut self,
        input: ReadCharacteristic,
        _context: &Context<Self>,
    ) -> Self::Result {
        let oneshot = {
            let session = self.sessions.get_mut(&input.accessory_id).unwrap();
            session
                .send(SessionReadCharacteristic {
                    service_name: input.service_name,
                    characteristic_name: input.characteristic_name,
                })
                .await??
        };
        Ok(oneshot.await?)
    }
}

#[async_trait]
impl Handler<IsConnected> for HiveProviderActor {
    type Result = bool;

    async fn handle(&mut self, input: IsConnected, _context: &Context<Self>) -> Self::Result {
        self.sessions.contains_key(&input.accessory_id)
    }
}

#[async_trait]
impl Provider for HiveProvider {
    async fn run(&self) -> Result<(), Error> {
        loop {
            let (stream, _) = self.tcp_listener.accept().await?;
            let events = self.global_events.clone();
            let mut address = self.address.clone();
            tokio::spawn(async move {
                let stream = tokio_tungstenite::accept_async(stream).await?;
                let (tx, mut rx) = stream.split();
                let NewSession {
                    accessory,
                    mut session,
                } = address.send(Connected { sink: tx }).await??;
                let accessory_id = accessory.id;
                events.send(Event::Connected { accessory })?;

                {
                    let session_cloned = session.clone();
                    let events_cloned = events.clone();
                    tokio::select! {
                        _ = tokio::spawn(async move {
                            while let Some(message) = rx.next().await {
                                let message = message?;
                                let response = session.send(message).await??;
                                if let Some(event) = response {
                                    events_cloned.send(event)?;
                                }

                            }
                            Ok::<(), anyhow::Error>(())
                        }) => {
                            tracing::debug!("stream has been closed");
                        }
                        _ = tokio::spawn(async move { session_cloned.wait_for_stop().await; }) => {
                            tracing::debug!("actor has stopped");
                        }

                    }
                }
                tracing::info!("session closed");
                address.send(Disconnected { accessory_id }).await?;
                events.send(Event::Disconnected { accessory_id })?;
                Ok::<(), anyhow::Error>(())
            });
        }
    }

    async fn write_characteristic(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceDiscriminants,
        characteristic: &Characteristic,
    ) -> Result<Result<(), accessory::Error>, Error> {
        self.address
            .clone()
            .send(WriteCharacteristic {
                accessory_id: *accessory_id,
                service_name: service_name.to_owned(),
                characteristic: characteristic.to_owned(),
            })
            .await
            .unwrap()
    }

    async fn read_characteristic(
        &self,
        accessory_id: &accessory::ID,
        service_name: &ServiceDiscriminants,
        characteristic_name: &CharacteristicDiscriminants,
    ) -> Result<Result<Characteristic, accessory::Error>, Error> {
        self.address
            .clone()
            .send(ReadCharacteristic {
                accessory_id: *accessory_id,
                service_name: service_name.to_owned(),
                characteristic_name: characteristic_name.to_owned(),
            })
            .await
            .unwrap()
    }

    async fn is_connected(&self, accessory_id: &accessory::ID) -> bool {
        self.address
            .clone()
            .send(IsConnected {
                accessory_id: *accessory_id,
            })
            .await
            .unwrap()
    }

    fn name(&self) -> &'static str {
        "mijia"
    }
}
pub struct SessionActor {
    accessory_id: accessory::ID,
    characteristic_write_results:
        HashMap<hive::FrameID, oneshot::Sender<Result<(), accessory::Error>>>,
    characteristic_read_results: HashMap<
        hive::FrameID,
        oneshot::Sender<Result<accessory::characteristics::Characteristic, accessory::Error>>,
    >,
    tx: SplitSink<WebSocketStream<TcpStream>, tungstenite::Message>,
}

#[async_trait]
impl Actor for SessionActor {}

#[async_trait]
impl Handler<tungstenite::Message> for SessionActor {
    type Result = Result<Option<Event>, Error>;

    async fn handle(
        &mut self,
        message: tungstenite::Message,
        context: &Context<Self>,
    ) -> Self::Result {
        match message {
            tungstenite::Message::Text(text) => {
                tracing::debug!(?text, "[->] text ...");
                let frame = serde_json::from_str::<hive::AccessoryFrame>(&text)?;
                tracing::debug!(?frame, "[->] ... frame");

                match frame {
                    hive::AccessoryFrame::CharacteristicReadResponse(frame) => {
                        self.characteristic_read_results
                            .remove(&frame.id)
                            .unwrap()
                            .send(frame.result.into())
                            .unwrap();
                        Ok(None)
                    }
                    hive::AccessoryFrame::CharacteristicWriteResponse(frame) => {
                        self.characteristic_write_results
                            .remove(&frame.id)
                            .unwrap()
                            .send(frame.result.into())
                            .unwrap();
                        Ok(None)
                    }
                    hive::AccessoryFrame::CharacteristicUpdate(frame) => {
                        Ok(Some(Event::CharacteristicUpdate {
                            accessory_id: self.accessory_id,
                            service_name: frame.service_name,
                            characteristic: frame.characteristic,
                        }))

                    },
                }
            }
            tungstenite::Message::Binary(bytes) => {
                tracing::debug!(?bytes, "[->] binary");
                Err(anyhow::anyhow!("unexpected binary message = {:?}", bytes))
            }
            tungstenite::Message::Ping(bytes) => {
                tracing::debug!(?bytes, "[->] ping");
                self.tx
                    .send(tungstenite::Message::Pong(bytes.clone()))
                    .await?;
                tracing::debug!(?bytes, "[<-] pong");
                Ok(None)
            }
            tungstenite::Message::Pong(bytes) => {
                tracing::debug!(?bytes, "[->] pong");
                Ok(None)
            }
            tungstenite::Message::Close(frame) => {
                tracing::debug!(?frame, "[->] close");
                context.address().stop().await;
                Ok(None)
            }
        }
    }
}

#[derive(Debug)]
struct SessionReadCharacteristic {
    service_name: accessory::services::ServiceDiscriminants,
    characteristic_name: accessory::characteristics::CharacteristicDiscriminants,
}

#[derive(Debug)]
struct SessionWriteCharacteristic {
    service_name: accessory::services::ServiceDiscriminants,
    characteristic: accessory::characteristics::Characteristic,
}

#[async_trait]
impl Handler<SessionReadCharacteristic> for SessionActor {
    type Result = Result<oneshot::Receiver<Result<Characteristic, accessory::Error>>, Error>;

    async fn handle(
        &mut self,
        input: SessionReadCharacteristic,
        _context: &Context<Self>,
    ) -> Self::Result {
        let frame_id = rand::random();
        let frame = hive::HubFrame::CharacteristicRead(hive::CharacteristicRead {
            id: frame_id,
            service_name: input.service_name,
            characteristic_name: input.characteristic_name,
        });
        let text = serde_json::to_string(&frame)?;
        let message = tungstenite::Message::Text(text);
        let (response_tx, response_rx) = oneshot::channel();
        self.characteristic_read_results
            .insert(frame_id, response_tx);
        self.tx.send(message).await?;
        Ok(response_rx)
    }
}

#[async_trait]
impl Handler<SessionWriteCharacteristic> for SessionActor {
    type Result = Result<oneshot::Receiver<Result<(), accessory::Error>>, Error>;

    async fn handle(
        &mut self,
        input: SessionWriteCharacteristic,
        _context: &Context<Self>,
    ) -> Self::Result {
        let frame_id = rand::random();
        let frame = hive::HubFrame::CharacteristicWrite(hive::CharacteristicWrite {
            id: frame_id,
            service_name: input.service_name,
            characteristic: input.characteristic,
        });
        let text = serde_json::to_string(&frame)?;
        let message = tungstenite::Message::Text(text);
        let (response_tx, response_rx) = oneshot::channel();
        self.characteristic_write_results
            .insert(frame_id, response_tx);
        self.tx.send(message).await?;
        Ok(response_rx)
    }
}

impl SessionActor {
    pub fn new(
        accessory_id: accessory::ID,
        tx: SplitSink<WebSocketStream<TcpStream>, tungstenite::Message>,
    ) -> Self {
        Self {
            accessory_id,
            tx,
            characteristic_write_results: Default::default(),
            characteristic_read_results: Default::default(),
        }
    }
}
