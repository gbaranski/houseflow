use super::Provider;
use anyhow::Error;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use futures::StreamExt;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::MijiaService as Config;
use houseflow_config::hub::AccessoryType;
use houseflow_types::device::ID as AccessoryID;
use mijia::bluetooth::DeviceId as BluetoothDeviceID;
use mijia::{MijiaEvent, MijiaSession};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MijiaProvider {
    connected_accessories: ArcSwap<HashMap<BluetoothDeviceID, AccessoryID>>,
    configured_accessories: Vec<Accessory>,
    mijia_session: MijiaSession,
}

impl MijiaProvider {
    pub async fn new(_config: Config, configured_accessories: Vec<Accessory>) -> Result<Self, Error> {
        let (_, mijia_session) = MijiaSession::new().await?;
        Ok(Self {
            connected_accessories: Default::default(),
            configured_accessories,
            mijia_session,
        })
    }

    fn accessory_id_by_bluetooth_device_id(
        &self,
        bluetooth_device_id: &BluetoothDeviceID,
    ) -> Option<AccessoryID> {
        self.connected_accessories.load().get(bluetooth_device_id).cloned()
    }

    fn accessory_by_mac_address(&self, mac_address: &str) -> Option<&Accessory> {
        self.configured_accessories.iter().find(|accessory| {
            if let AccessoryType::XiaomiMijia {
                mac_address: accessory_mac_address,
            } = &accessory.r#type
            {
                accessory_mac_address == mac_address
            } else {
                false
            }
        })
    }

    #[tracing::instrument(skip(self, accessory, bluetooth_device_id), fields(id = %accessory.id))]
    async fn connect(
        &self,
        accessory: &Accessory,
        bluetooth_device_id: &BluetoothDeviceID,
    ) -> Result<(), Error> {
        tracing::info!("connecting");
        self.mijia_session
            .bt_session
            .connect(bluetooth_device_id)
            .await?;
        tracing::info!("connected");
        self.mijia_session
            .start_notify_sensor(bluetooth_device_id)
            .await?;
        {
            let mut new_connected_accessories = self.connected_accessories.load().as_ref().clone();
            new_connected_accessories.insert(bluetooth_device_id.clone(), accessory.id.clone());
            self.connected_accessories
                .store(Arc::new(new_connected_accessories));
        }
        todo!()
    }
}

#[async_trait]
impl Provider for MijiaProvider {
    async fn run(&self) -> Result<(), Error> {
        self.mijia_session.bt_session.start_discovery().await?;
        let sensors = self.mijia_session.get_sensors().await?;
        for sensor in sensors {
            let accessory = self.accessory_by_mac_address(sensor.mac_address.to_string().as_str());
            if let Some(accessory) = accessory {
                tracing::info!(mac = %sensor.mac_address, id = %accessory.id, "discovered");
                if let Err(err) = self.connect(accessory, &sensor.id).await {
                    tracing::info!(mac = %sensor.mac_address, id = %accessory.id, "connect failed due to {}", err);
                }
            } else {
                tracing::info!(mac = %sensor.mac_address, "discovered, skipping");
            }
        }

        let mut stream = self.mijia_session.event_stream().await?;
        while let Some(event) = stream.next().await {
            tracing::debug!("received event = {:?}", event);
            match event {
                MijiaEvent::Discovered { id } => {
                    tracing::info!("discovered: {}", id);
                }
                MijiaEvent::Readings { id, readings } => {
                    let accessory_id = self.accessory_id_by_bluetooth_device_id(&id).unwrap();
                    tracing::info!("readings from {} = {}", accessory_id, readings);
                }
                MijiaEvent::HistoryRecord { id, record } => {
                    let accessory_id = self.accessory_id_by_bluetooth_device_id(&id).unwrap();
                    tracing::info!("new history record from {} = {}", accessory_id, record);
                }
                MijiaEvent::Disconnected { id } => {
                    let accessory_id = self.accessory_id_by_bluetooth_device_id(&id).unwrap();
                    tracing::info!("{} disconnected", accessory_id);
                    {
                        let mut new_connected_accessories =
                            self.connected_accessories.load().as_ref().clone();
                        new_connected_accessories.remove(&id);
                        self.connected_accessories
                            .store(Arc::new(new_connected_accessories));
                    }
                }
                _ => todo!(),
            };
        }
        Ok(())
    }

    async fn discover(&self) -> Result<Option<super::Accessory>, Error> {
        todo!()
    }

    fn name(&self) -> &'static str {
        "mijia"
    }
}
