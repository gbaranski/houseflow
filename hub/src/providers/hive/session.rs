use crate::providers::Event;
use anyhow::Error;
use async_trait::async_trait;
use futures::stream::SplitSink;
use futures::SinkExt;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::hive;
use ::messages::prelude::*;
use std::collections::HashMap;
use tokio::sync::oneshot;
use axum::extract::ws;
use axum::extract::ws::WebSocket;

pub type Address = ::messages::prelude::Address<Session>;

pub(super) mod messages {
    use super::*;

    #[derive(Debug)]
    pub struct ReadCharacteristic {
        pub service_name: accessory::services::ServiceName,
        pub characteristic_name: accessory::characteristics::CharacteristicDiscriminants,
    }

    #[derive(Debug)]
    pub struct WriteCharacteristic {
        pub service_name: accessory::services::ServiceName,
        pub characteristic: accessory::characteristics::Characteristic,
    }
}

pub struct Session {
    accessory_id: accessory::ID,
    characteristic_write_results:
        HashMap<hive::FrameID, oneshot::Sender<Result<(), accessory::Error>>>,
    characteristic_read_results: HashMap<
        hive::FrameID,
        oneshot::Sender<Result<accessory::characteristics::Characteristic, accessory::Error>>,
    >,
    tx: SplitSink<WebSocket, ws::Message>,
}

#[async_trait]
impl Actor for Session {}

#[async_trait]
impl Handler<ws::Message> for Session {
    type Result = Result<Option<Event>, Error>;

    async fn handle(
        &mut self,
        message: ws::Message,
        context: &Context<Self>,
    ) -> Self::Result {
        match message {
            ws::Message::Text(text) => {
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
                    }
                }
            }
            ws::Message::Binary(bytes) => {
                tracing::debug!(?bytes, "[->] binary");
                Err(anyhow::anyhow!("unexpected binary message = {:?}", bytes))
            }
            ws::Message::Ping(bytes) => {
                tracing::debug!(?bytes, "[->] ping");
                self.tx
                    .send(ws::Message::Pong(bytes.clone()))
                    .await?;
                tracing::debug!(?bytes, "[<-] pong");
                Ok(None)
            }
            ws::Message::Pong(bytes) => {
                tracing::debug!(?bytes, "[->] pong");
                Ok(None)
            }
            ws::Message::Close(frame) => {
                tracing::debug!(?frame, "[->] close");
                context.address().stop().await;
                Ok(None)
            }
        }
    }
}

#[async_trait]
impl Handler<messages::ReadCharacteristic> for Session {
    type Result = Result<oneshot::Receiver<Result<Characteristic, accessory::Error>>, Error>;

    async fn handle(
        &mut self,
        input: messages::ReadCharacteristic,
        _context: &Context<Self>,
    ) -> Self::Result {
        let frame_id = rand::random();
        let frame = hive::HubFrame::CharacteristicRead(hive::CharacteristicRead {
            id: frame_id,
            service_name: input.service_name,
            characteristic_name: input.characteristic_name,
        });
        let text = serde_json::to_string(&frame)?;
        let message = ws::Message::Text(text);
        let (response_tx, response_rx) = oneshot::channel();
        self.characteristic_read_results
            .insert(frame_id, response_tx);
        self.tx.send(message).await?;
        Ok(response_rx)
    }
}

#[async_trait]
impl Handler<messages::WriteCharacteristic> for Session {
    type Result = Result<oneshot::Receiver<Result<(), accessory::Error>>, Error>;

    async fn handle(
        &mut self,
        input: messages::WriteCharacteristic,
        _context: &Context<Self>,
    ) -> Self::Result {
        let frame_id = rand::random();
        let frame = hive::HubFrame::CharacteristicWrite(hive::CharacteristicWrite {
            id: frame_id,
            service_name: input.service_name,
            characteristic: input.characteristic,
        });
        let text = serde_json::to_string(&frame)?;
        let message = ws::Message::Text(text);
        let (response_tx, response_rx) = oneshot::channel();
        self.characteristic_write_results
            .insert(frame_id, response_tx);
        self.tx.send(message).await?;
        Ok(response_rx)
    }
}

impl Session {
    pub fn new(
        accessory_id: accessory::ID,
        tx: SplitSink<WebSocket, ws::Message>,
    ) -> Self {
        Self {
            accessory_id,
            tx,
            characteristic_write_results: Default::default(),
            characteristic_read_results: Default::default(),
        }
    }
}