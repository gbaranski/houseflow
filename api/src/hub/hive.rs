use async_trait::async_trait;
use ezsockets::ClientConfig;
use houseflow_accessory_hal::Accessory;
use houseflow_config::accessory::Credentials;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::hive::AccessoryFrame;
use houseflow_types::hive::CharacteristicReadResult;
use houseflow_types::hive::CharateristicWriteResult;
use houseflow_types::hive::HubFrame;
use houseflow_types::hive::ReadCharacteristic;
use houseflow_types::hive::UpdateCharacteristic;
use houseflow_types::hive::WriteCharacteristic;
use reqwest::Url;

#[derive(Clone)]
pub struct HiveClient {
    client: ezsockets::Client<()>,
}

impl HiveClient {
    pub async fn connect<A: Accessory + Send + Sync + 'static>(
        accessory_fn: impl FnOnce(Self) -> A,
        credentials: Credentials,
        hub_url: Url,
    ) -> Result<(), ezsockets::Error> {
        let hive_url = hub_url.join("provider/hive/websocket").unwrap();
        let (_, future) = ezsockets::connect(
            |client| {
                let client = Self { client };
                let accessory = accessory_fn(client.clone());
                HiveClientActor { accessory, client }
            },
            ClientConfig::new(hive_url).basic(
                &credentials.id.to_string(),
                &credentials.password.to_string(),
            ),
        )
        .await;
        future.await?;
        Ok(())
    }

    async fn frame(&self, frame: AccessoryFrame) {
        let s = serde_json::to_string(&frame).unwrap();
        self.client.text(s).await;
    }

    pub async fn update(&self, service_name: ServiceName, characteristic: Characteristic) {
        self.frame(AccessoryFrame::UpdateCharacteristic(UpdateCharacteristic {
            service_name,
            characteristic,
        }))
        .await;
    }
}

struct HiveClientActor<A: Accessory + Send + Sync + 'static> {
    accessory: A,
    client: HiveClient,
}

#[async_trait]
impl<A: Accessory + Send + Sync + 'static> ezsockets::ClientExt for HiveClientActor<A> {
    type Params = ();

    async fn call(&mut self, params: Self::Params) -> Result<(), ezsockets::Error> {
        let () = params;
        Ok(())
    }

    async fn text(&mut self, text: String) -> Result<(), ezsockets::Error> {
        let frame = serde_json::from_str::<HubFrame>(&text)?;
        let frame = match frame {
            HubFrame::ReadCharacteristic(ReadCharacteristic {
                id,
                service_name,
                characteristic_name,
            }) => {
                let result = self
                    .accessory
                    .read_characteristic(service_name, characteristic_name)
                    .await;
                let frame = CharacteristicReadResult {
                    id,
                    result: result.into(),
                };
                Some(AccessoryFrame::CharacteristicReadResult(frame))
            }
            HubFrame::WriteCharacteristic(WriteCharacteristic {
                id,
                service_name,
                characteristic,
            }) => {
                let result = self
                    .accessory
                    .write_characteristic(service_name, characteristic)
                    .await;
                let frame = CharateristicWriteResult {
                    id,
                    result: result.into(),
                };
                Some(AccessoryFrame::CharacteristicWriteResult(frame))
            }
            _ => unimplemented!(),
        };
        if let Some(frame) = frame {
            self.client.frame(frame).await;
        }

        Ok(())
    }

    async fn binary(&mut self, _bytes: Vec<u8>) -> Result<(), ezsockets::Error> {
        unimplemented!()
    }
}
