pub use super::Handle;
use super::Name;

use super::Message;
use crate::providers;
use crate::providers::ProviderExt;
use async_trait::async_trait;
use houseflow_config::hub::controllers::Lighthouse as Config;
use houseflow_types::hub;
use houseflow_types::lighthouse;

pub struct LighthouseController {
    provider: providers::MasterHandle,
    client: ezsockets::Client<Message>,
}

impl LighthouseController {
    async fn send(&mut self, frame: lighthouse::HubFrame) -> anyhow::Result<()> {
        let json = serde_json::to_string(&frame)?;
        self.client.text(json).await;
        Ok(())
    }
}

#[async_trait]
impl ezsockets::ClientExt for LighthouseController {
    type Params = Message;

    async fn text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let frame = serde_json::from_str::<lighthouse::ServerFrame>(&text)?;
        match frame {
            lighthouse::ServerFrame::ReadCharacteristic(lighthouse::ReadCharacteristic {
                id,
                accessory_id,
                service_name,
                characteristic_name,
            }) => {
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
            lighthouse::ServerFrame::WriteCharacteristic(lighthouse::WriteCharacteristic {
                id,
                accessory_id,
                service_name,
                characteristic,
            }) => {
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
        Ok(())
    }

    async fn binary(&mut self, _bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        todo!()
    }

    async fn call(&mut self, params: Self::Params) -> Result<(), ezsockets::Error> {
        match params {
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

pub async fn new(
    config: Config,
    hub_id: hub::ID,
    provider: providers::MasterHandle,
) -> Result<Handle, anyhow::Error> {
    let (client, _) = ezsockets::connect(
        |client| LighthouseController { provider, client },
        ezsockets::ClientConfig::new(config.url).basic(&hub_id.to_string(), &config.password),
    )
    .await;

    let sender: tokio::sync::mpsc::UnboundedSender<Message> = client.into();
    let sender = acu::Sender::new_from_mpsc(sender, Name::Lighthouse);
    Ok(Handle { sender })
}
