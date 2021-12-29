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
struct Execute {
    accessory_id: accessory::ID,
    command: accessory::Command,
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
impl Handler<Execute> for HiveProviderActor {
    type Result = Result<(accessory::Status, accessory::State), Error>;

    async fn handle(&mut self, input: Execute, _context: &Context<Self>) -> Self::Result {
        let oneshot = {
            let session = self.sessions.get_mut(&input.accessory_id).unwrap();
            session.send(input.command).await??
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

    async fn execute(
        &self,
        accessory_id: accessory::ID,
        command: accessory::Command,
    ) -> Result<(accessory::Status, accessory::State), Error> {
        self.address
            .clone()
            .send(Execute {
                accessory_id,
                command,
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
    execute_results: HashMap<hive::FrameID, oneshot::Sender<(accessory::Status, accessory::State)>>,
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
                    hive::AccessoryFrame::State(frame) => Ok(Some(Event::State {
                        accessory_id: self.accessory_id,
                        state: frame.state,
                    })),
                    hive::AccessoryFrame::ExecuteResult(frame) => {
                        tracing::info!("execute result = {:?}", frame);
                        self.execute_results
                            .remove(&frame.id)
                            .unwrap()
                            .send((frame.status, frame.state))
                            .unwrap();
                        Ok(None)
                    }
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

#[async_trait]
impl Handler<accessory::Command> for SessionActor {
    type Result = Result<oneshot::Receiver<(accessory::Status, accessory::State)>, Error>;

    async fn handle(
        &mut self,
        input: accessory::Command,
        _context: &Context<Self>,
    ) -> Self::Result {
        let frame_id = rand::random();
        let frame = hive::HubFrame::Execute(hive::ExecuteFrame {
            id: frame_id,
            command: input.to_owned(),
        });
        let text = serde_json::to_string(&frame)?;
        let message = tungstenite::Message::Text(text);
        let (response_tx, response_rx) = oneshot::channel();
        self.execute_results.insert(frame_id, response_tx);
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
            execute_results: Default::default(),
        }
    }
}
