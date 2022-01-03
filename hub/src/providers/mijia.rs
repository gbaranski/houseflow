use super::Event;
use super::EventSender;
use super::Provider;
use anyhow::Error;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use futures::StreamExt;
use houseflow_config::hub::manufacturers;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::AccessoryType;
use houseflow_config::hub::MijiaProvider as Config;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::services::ServiceDiscriminants;
use houseflow_types::accessory::ID as AccessoryID;
use mijia::bluetooth::DeviceId as BluetoothDeviceID;
use mijia::{MijiaEvent, MijiaSession};
use std::collections::HashMap;
use std::sync::Arc;

pub struct MijiaProvider {
    connected_accessories: ArcSwap<HashMap<BluetoothDeviceID, AccessoryID>>,
    configured_accessories: Vec<Accessory>,
    events: EventSender,
    mijia_session: MijiaSession,
}

impl MijiaProvider {
    pub async fn new(
        _config: Config,
        configured_accessories: Vec<Accessory>,
        events: EventSender,
    ) -> Result<Self, Error> {
        let (_, mijia_session) = MijiaSession::new().await?;
        Ok(Self {
            connected_accessories: Default::default(),
            configured_accessories,
            mijia_session,
            events,
        })
    }

    fn accessory_id_by_bluetooth_device_id(
        &self,
        bluetooth_device_id: &BluetoothDeviceID,
    ) -> Option<AccessoryID> {
        self.connected_accessories
            .load()
            .get(bluetooth_device_id)
            .cloned()
    }

    fn bluetooth_device_id_by_accessory_id(
        &self,
        accessory_id: &accessory::ID,
    ) -> Option<BluetoothDeviceID> {
        self.connected_accessories.load().iter().find_map(
            |(current_bluetooth_device_id, current_accessory_id)| {
                if current_accessory_id == accessory_id {
                    Some(current_bluetooth_device_id.clone())
                } else {
                    None
                }
            },
        )
    }

    fn accessory_by_mac_address(&self, expected_mac_address: &str) -> Option<&Accessory> {
        self.configured_accessories.iter().find(|accessory| {
            if let AccessoryType::XiaomiMijia(manufacturers::XiaomiMijia::HygroThermometer {
                mac_address,
            }) = &accessory.r#type
            {
                expected_mac_address == mac_address
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
        self.events
            .send(Event::Connected {
                accessory: accessory.to_owned(),
            })
            .unwrap();
        Ok(())
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
                    self.events
                        .send(Event::CharacteristicUpdate {
                            accessory_id,
                            service_name: ServiceDiscriminants::TemperatureSensor,
                            characteristic: Characteristic::CurrentTemperature(
                                characteristics::CurrentTemperature {
                                    temperature: readings.temperature,
                                },
                            ),
                        })
                        .unwrap();
                    self.events
                        .send(Event::CharacteristicUpdate {
                            accessory_id,
                            service_name: ServiceDiscriminants::HumiditySensor,
                            characteristic: Characteristic::CurrentHumidity(
                                characteristics::CurrentHumidity {
                                    humidity: readings.humidity as f32,
                                },
                            ),
                        })
                        .unwrap();
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

    async fn write_characteristic(
        &self,
        _accessory_id: &accessory::ID,
        _service_name: &accessory::services::ServiceDiscriminants,
        _characteristic: &accessory::characteristics::Characteristic,
    ) -> Result<Result<(), accessory::Error>, Error> {
        Ok(Err(accessory::Error::CharacteristicReadOnly))
    }

    async fn read_characteristic(
        &self,
        _accessory_id: &accessory::ID,
        _service_name: &accessory::services::ServiceDiscriminants,
        _characteristic_name: &accessory::characteristics::CharacteristicDiscriminants,
    ) -> Result<Result<Characteristic, accessory::Error>, Error> {
        todo!()
        // match service_name {
        //     accessory::services::ServiceDiscriminants::TemperatureSensor => todo!(),
        //     accessory::services::ServiceDiscriminants::HumiditySensor => todo!(),
        //     _ => return Ok(Err(accessory::Error::ServiceNotSupported)),
        // };

        // let characteristic = match characteristic_name {
        //     CharacteristicDiscriminants::CurrentTemperature => {
        //         Characteristic::CurrentTemperature(characteristics::CurrentTemperature {
        //             temperature: 10.5,
        //         })
        //     }
        //     CharacteristicDiscriminants::CurrentHumidity => {
        //         Characteristic::CurrentHumidity(characteristics::CurrentHumidity { humidity: 50.0 })
        //     }
        //     _ => return Ok(Err(accessory::Error::CharacteristicNotSupported)),
        // };

        // Ok(Ok(characteristic))
    }

    async fn is_connected(&self, accessory_id: &accessory::ID) -> bool {
        self.bluetooth_device_id_by_accessory_id(&accessory_id)
            .is_some()
    }

    fn name(&self) -> &'static str {
        "mijia"
    }
}