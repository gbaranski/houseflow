use super::Handle;
use super::Message;
use crate::controllers::Name;
use crate::providers::ProviderExt;
use anyhow::Context;
use futures::SinkExt;
use futures::StreamExt;
use houseflow_config::hub::controllers::Lighthouse as Config;
use houseflow_types::hub;
use houseflow_types::lighthouse;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::Message as WebSocketMessage;
use tokio_tungstenite::WebSocketStream;

pub struct LighthouseController<P: ProviderExt> {
    receiver: acu::Receiver<Message, Name>,
    websocket_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    provider: P,
}

pub async fn new(
    provider: impl ProviderExt + Send + Sync + 'static,
    hub_id: hub::ID,
    config: Config,
) -> Result<Handle, anyhow::Error> {
    let (sender, receiver) = acu::channel(1, Name::Lighthouse);
    tracing::debug!(
        "attempting to connect to the lighthouse websocket server on URL: {}",
        config.url
    );

    let authorization_header = format!(
        "Basic {}",
        base64::encode(format!(
            "{}:{}",
            hub_id.to_string().as_str(),
            config.password
        ))
    );

    let request = http::Request::builder()
        .uri(config.url.as_str())
        .header(http::header::AUTHORIZATION, authorization_header)
        .body(())
        .unwrap();

    let (websocket_stream, websocket_response) = tokio_tungstenite::connect_async(request)
        .await
        .context("lighthouse websocket server connect failed")?;
    tracing::debug!(
        "connected to the lighthouse server via websocket with response: {:?}",
        websocket_response
    );

    let handle = Handle { sender };
    let mut actor = LighthouseController {
        receiver,
        websocket_stream,
        provider,
    };
    tokio::spawn(async move { actor.run().await });
    Ok(handle)
}

impl<P: ProviderExt> LighthouseController<P> {
    async fn send(&mut self, frame: lighthouse::HubFrame) -> Result<(), anyhow::Error> {
        let text = serde_json::to_string(&frame).context("serializing outgoing hub frame")?;
        self.websocket_stream
            .send(tungstenite::Message::Text(text))
            .await?;
        Ok(())
    }

    async fn handle_websocket_message(
        &mut self,
        message: tungstenite::Message,
    ) -> Result<(), anyhow::Error> {
        match message {
            WebSocketMessage::Text(text) => {
                let frame = serde_json::from_str::<lighthouse::ServerFrame>(&text)?;
                match frame {
                    lighthouse::ServerFrame::ReadCharacteristic(
                        lighthouse::ReadCharacteristic {
                            id,
                            accessory_id,
                            service_name,
                            characteristic_name,
                        },
                    ) => {
                        let result = self
                            .provider
                            .read_characteristic(accessory_id, service_name, characteristic_name)
                            .await
                            .into();
                        self.send(lighthouse::HubFrame::ReadCharacteristicResult(
                            lighthouse::ReadCharacteristicResult { id, result },
                        ))
                        .await?;
                    }
                    lighthouse::ServerFrame::WriteCharacteristic(
                        lighthouse::WriteCharacteristic {
                            id,
                            accessory_id,
                            service_name,
                            characteristic,
                        },
                    ) => {
                        let result = self
                            .provider
                            .write_characteristic(accessory_id, service_name, characteristic)
                            .await
                            .into();
                        self.send(lighthouse::HubFrame::WriteCharacteristicResult(
                            lighthouse::WriteCharacteristicResult { id, result },
                        ))
                        .await?;
                    }
                    _ => unimplemented!(),
                }
            }
            WebSocketMessage::Binary(_) => todo!(),
            WebSocketMessage::Ping(_) => todo!(),
            WebSocketMessage::Pong(_) => todo!(),
            WebSocketMessage::Close(_) => todo!(),
        };
        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        loop {
            tokio::select! {
                Some(message) = self.receiver.recv() => {
                    self.handle_controller_message(message).await?;
                },
                Some(message) = self.websocket_stream.next() => {
                    let message = message?;
                    self.handle_websocket_message(message).await?
                },
                else => break,
            }
        }

        Ok(())
    }

    async fn handle_controller_message(&mut self, message: Message) -> Result<(), anyhow::Error> {
        match message {
            Message::Connected { accessory } => {
                self.send(lighthouse::HubFrame::AccessoryConnected(accessory.into()))
                    .await?;
            }
            Message::Disconnected { accessory_id } => {
                self.send(lighthouse::HubFrame::AccessoryDisconnected(accessory_id))
                    .await?;
            }
            Message::Updated {
                accessory_id: _,
                service_name: _,
                characteristic: _,
            } => {}
        };
        Ok(())
    }
}
