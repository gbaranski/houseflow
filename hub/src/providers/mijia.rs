use crate::controllers;
use crate::controllers::ControllerExt;
use anyhow::Error;
use futures::StreamExt;
use houseflow_config::hub::manufacturers;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::AccessoryType;
use houseflow_config::hub::MijiaProvider as Config;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::accessory::ID as AccessoryID;
use mijia::bluetooth::DeviceId as BluetoothDeviceID;
use mijia::MijiaEvent;
use mijia::MijiaSession;
use std::collections::HashMap;

pub use super::Handle;
use super::Message;
use super::Name;

pub async fn new(
    _config: Config,
    controller: controllers::MasterHandle,
    configured_accessories: Vec<Accessory>,
) -> Result<Handle, Error> {
    let (sender, receiver) = acu::channel(8, Name::Mijia);

    let (_, mijia_session) = MijiaSession::new().await?;
    let mut actor = MijiaProvider {
        receiver,
        controller,
        connected_accessories: Default::default(),
        configured_accessories,
        last_readings: Default::default(),
        mijia_session,
    };

    let handle = Handle { sender };
    tokio::spawn(async move { actor.run().await });
    Ok(handle)
}

pub struct MijiaProvider {
    receiver: acu::Receiver<Message, Name>,
    controller: controllers::MasterHandle,
    connected_accessories: HashMap<BluetoothDeviceID, AccessoryID>,
    configured_accessories: Vec<Accessory>,
    last_readings: HashMap<AccessoryID, mijia::Readings>,
    mijia_session: MijiaSession,
}

impl MijiaProvider {
    async fn run(&mut self) -> Result<(), anyhow::Error> {
        self.mijia_session.bt_session.start_discovery().await?;
        let sensors = self.mijia_session.get_sensors().await?;
        for sensor in sensors {
            let accessory = self
                .accessory_by_mac_address(sensor.mac_address.to_string().as_str())
                .cloned();
            if let Some(accessory) = accessory {
                tracing::info!(mac = %sensor.mac_address, id = %accessory.id, "discovered");
                if let Err(err) = self.connect(accessory.clone(), &sensor.id).await {
                    tracing::info!(mac = %sensor.mac_address, id = %accessory.id, "connect failed due to {}", err);
                }
            } else {
                tracing::info!(mac = %sensor.mac_address, "discovered, skipping");
            }
        }
        let mut mijia_events = self.mijia_session.event_stream().await?;

        loop {
            tokio::select! {
                Some(event) = mijia_events.next() => {
                    self.handle_mijia_event(event).await?;
                },
                Some(message) = self.receiver.recv() => {
                    self.handle_provider_message(message).await?;
                }
                else => break,
            }
        }

        Ok(())
    }

    fn accessory_id_by_bluetooth_device_id(
        &self,
        bluetooth_device_id: &BluetoothDeviceID,
    ) -> Option<AccessoryID> {
        self.connected_accessories.get(bluetooth_device_id).cloned()
    }

    fn bluetooth_device_id_by_accessory_id(
        &self,
        accessory_id: &accessory::ID,
    ) -> Option<BluetoothDeviceID> {
        self.connected_accessories.iter().find_map(
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
        &mut self,
        accessory: Accessory,
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
        self.connected_accessories
            .insert(bluetooth_device_id.clone(), accessory.id);
        self.controller.connected(accessory).await;
        Ok(())
    }

    async fn handle_mijia_event(&mut self, event: MijiaEvent) -> Result<(), Error> {
        tracing::debug!("received event = {:?}", event);
        match event {
            MijiaEvent::Discovered { id } => {
                tracing::info!("discovered: {}", id);
            }
            MijiaEvent::Readings { id, readings } => {
                let accessory_id = self.accessory_id_by_bluetooth_device_id(&id).unwrap();
                tracing::info!("readings from {} = {}", accessory_id, readings);
                self.last_readings.insert(accessory_id, readings.clone());
                self.controller
                    .updated(
                        accessory_id,
                        ServiceName::TemperatureSensor,
                        Characteristic::CurrentTemperature(characteristics::CurrentTemperature {
                            temperature: readings.temperature,
                        }),
                    )
                    .await;
                self.controller
                    .updated(
                        accessory_id,
                        ServiceName::HumiditySensor,
                        Characteristic::CurrentHumidity(characteristics::CurrentHumidity {
                            humidity: readings.humidity as f32,
                        }),
                    )
                    .await;
                self.controller
                    .updated(
                        accessory_id,
                        ServiceName::Battery,
                        Characteristic::BatteryLevel(characteristics::BatteryLevel {
                            battery_level_percent: readings.battery_percent as u8,
                        }),
                    )
                    .await;
            }
            MijiaEvent::HistoryRecord { id, record } => {
                let accessory_id = self.accessory_id_by_bluetooth_device_id(&id).unwrap();
                tracing::info!("new history record from {} = {}", accessory_id, record);
            }
            MijiaEvent::Disconnected { id } => {
                let accessory_id = self.accessory_id_by_bluetooth_device_id(&id).unwrap();
                tracing::info!("{} disconnected", accessory_id);
                self.last_readings.remove(&accessory_id);
                self.connected_accessories.remove(&id);
                self.controller.disconnected(accessory_id).await;
            }
            _ => todo!(),
        };
        Ok(())
    }

    async fn handle_provider_message(&mut self, message: Message) -> Result<(), Error> {
        match message {
            Message::WriteCharacteristic {
                accessory_id: _,
                service_name: _,
                characteristic: _,
                respond_to,
            } => {
                respond_to
                    .send(Err(accessory::Error::CharacteristicNotSupported))
                    .unwrap();
            }
            Message::ReadCharacteristic {
                accessory_id,
                service_name,
                characteristic_name,
                respond_to,
            } => {
                let last_readings = self.last_readings.get(&accessory_id).unwrap().clone();

                let characteristic = match service_name {
                    accessory::services::ServiceName::TemperatureSensor => {
                        if characteristic_name == CharacteristicName::CurrentTemperature {
                            Ok(Characteristic::CurrentTemperature(
                                characteristics::CurrentTemperature {
                                    temperature: last_readings.temperature,
                                },
                            ))
                        } else {
                            Err(accessory::Error::CharacteristicNotSupported)
                        }
                    }
                    accessory::services::ServiceName::HumiditySensor => {
                        if characteristic_name == CharacteristicName::CurrentHumidity {
                            Ok(Characteristic::CurrentHumidity(
                                characteristics::CurrentHumidity {
                                    humidity: last_readings.humidity as f32,
                                },
                            ))
                        } else {
                            Err(accessory::Error::CharacteristicNotSupported)
                        }
                    }
                    _ => Err(accessory::Error::ServiceNotSupported),
                };
                respond_to.send(characteristic).unwrap();
            }
            Message::IsConnected {
                accessory_id,
                respond_to,
            } => {
                respond_to
                    .send(
                        self.bluetooth_device_id_by_accessory_id(&accessory_id)
                            .is_some(),
                    )
                    .unwrap();
            }
            Message::GetAccessoryConfiguration {
                accessory_id,
                respond_to,
            } => {
                respond_to
                    .send(
                        self.configured_accessories
                            .iter()
                            .find(|accessory| accessory.id == accessory_id)
                            .cloned(),
                    )
                    .unwrap();
            }
        };

        Ok(())
    }
}
