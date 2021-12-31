use super::Event;
use super::EventSender;
use super::Controller;
use anyhow::Error;
use async_trait::async_trait;
use atomic::AtomicU64;
use atomic::Ordering;
use futures::lock::Mutex;
use futures::FutureExt;
use hap::accessory::garage_door_opener::GarageDoorOpenerAccessory;
use hap::accessory::temperature_sensor::TemperatureSensorAccessory;
use hap::accessory::AccessoryCategory;
use hap::accessory::AccessoryInformation;
use hap::accessory::HapAccessory;
use hap::characteristic::AsyncCharacteristicCallbacks;
use hap::characteristic::CharacteristicCallbacks;
use hap::server::IpServer;
use hap::server::Server;
use hap::storage::FileStorage;
use hap::storage::Storage;
use hap::HapType;
use hap::MacAddress;
use hap::Pin;
use houseflow_config::hub::manufacturers;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::AccessoryType;
pub use houseflow_config::hub::HapController as HapConfig;
use houseflow_types::accessory;
use mac_address::get_mac_address;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::atomic;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HapController {
    ip_server: IpServer,
    accessory_pointers: RwLock<HashMap<accessory::ID, Arc<Mutex<Box<dyn HapAccessory>>>>>,
    last_accessory_instace_id: AtomicU64,
    events: EventSender,
}

impl HapController {
    pub async fn new(config: &HapConfig, events: EventSender) -> Result<Self, Error> {
        let mut storage =
            FileStorage::new(&houseflow_config::defaults::data_home().join("hap")).await?;
        let config = match storage.load_config().await {
            Ok(mut config) => {
                config.redetermine_local_ip();
                storage.save_config(&config).await?;
                config
            }
            Err(_) => {
                let pin = config
                    .pin
                    .chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
                    .as_slice()
                    .try_into()
                    .unwrap();
                hap::Config {
                    pin: Pin::new(pin)?,
                    name: config.name.clone(),
                    device_id: MacAddress::from_bytes(&get_mac_address().unwrap().unwrap().bytes())
                        .unwrap(),
                    category: AccessoryCategory::Bridge,
                    ..Default::default()
                }
            }
        };

        storage.save_config(&config).await?;
        Ok(Self {
            ip_server: IpServer::new(config, storage).await?,
            accessory_pointers: Default::default(),
            last_accessory_instace_id: AtomicU64::from(1),
            events,
        })
    }
}

#[async_trait]
impl Controller for HapController {
    async fn run(&self) -> Result<(), Error> {
        self.ip_server.run_handle().await?;
        Ok(())
    }

    async fn connected(&self, configured_accessory: &Accessory) -> Result<(), Error> {
        let accessory_instance_id = self
            .last_accessory_instace_id
            .fetch_add(1, Ordering::Relaxed);

        let accessory_ptr = match &configured_accessory.r#type {
            AccessoryType::XiaomiMijia(accessory_type) => {
                use manufacturers::XiaomiMijia as Manufacturer;

                let manufacturer = "Xiaomi Mijia".to_string();
                match accessory_type {
                    Manufacturer::HygroThermometer { mac_address: _ } => {
                        let mut temperature_sensor = TemperatureSensorAccessory::new(
                            accessory_instance_id,
                            AccessoryInformation {
                                manufacturer,
                                model: "LYWSD03MMC".to_string(), // TODO: ensure that this one is okay
                                name: "Thermometer".to_string(),
                                serial_number: configured_accessory.id.to_string(),
                                accessory_flags: None,
                                application_matching_identifier: None,
                                // configured_name: Some(configured_accessory.name.clone()), For some reason it causes the Home app to break
                                configured_name: None,
                                firmware_revision: None,
                                hardware_finish: None,
                                hardware_revision: None,
                                product_data: None,
                                software_revision: None,
                            },
                        )?;
                        temperature_sensor
                            .temperature_sensor
                            .current_temperature
                            .on_read(Some(|| Ok(None)));

                        self.ip_server.add_accessory(temperature_sensor).await?
                    }
                    _ => unimplemented!(),
                }
            }
            AccessoryType::Houseflow(accessory_type) => {
                use manufacturers::Houseflow as Manufacturer;

                let manufacturer = "Houseflow".to_string();
                match accessory_type {
                    Manufacturer::Garage => {
                        let mut garage_door_opener = GarageDoorOpenerAccessory::new(
                            accessory_instance_id,
                            AccessoryInformation {
                                manufacturer,
                                model: "houseflow-garage".to_string(), // TODO: ensure that this one is okay
                                name: "Garage".to_string(),
                                serial_number: configured_accessory.id.to_string(),
                                accessory_flags: None,
                                application_matching_identifier: None,
                                // configured_name: Some(configured_accessory.name.clone()), For some reason it causes the Home app to break
                                configured_name: None,
                                firmware_revision: None,
                                hardware_finish: None,
                                hardware_revision: None,
                                product_data: None,
                                software_revision: None,
                            },
                        )?;
                        garage_door_opener
                            .garage_door_opener
                            .current_door_state
                            .on_read(Some(|| Ok(None)));

                        let events = self.events.clone();

                        let accessory_id = configured_accessory.id;
                        garage_door_opener
                            .garage_door_opener
                            .target_door_state
                            .on_update_async(Some(move |current: u8, new: u8| {
                                let events = events.clone();

                                async move {
                                    println!("garage_door_opener target door state characteristic updated from {} to {}", current, new);
                                    events
                                        .send(Event::Execute(
                                            accessory_id,
                                            accessory::Command::OpenClose(
                                                accessory::commands::OpenClose {
                                                    open_percent: if new == 1 {
                                                        100
                                                    } else if new == 0 {
                                                        0
                                                    } else {
                                                        unreachable!()
                                                    },
                                                },
                                            ),
                                        ))
                                        .unwrap();
                                    Ok(())
                                }
                                .boxed()
                            }));

                        tracing::info!("registering new garage door opener accessory");
                        self.ip_server.add_accessory(garage_door_opener).await?
                    }
                    Manufacturer::Gate => todo!(),
                    _ => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        };
        let mut accessory_pointers = self.accessory_pointers.write().await;
        accessory_pointers.insert(configured_accessory.id, accessory_ptr);
        Ok(())
    }

    async fn update_state(
        &self,
        id: &accessory::ID,
        state: &accessory::State,
    ) -> Result<(), Error> {
        tracing::debug!("updating state of {} to {:?}", id, state);
        let accessory_pointers = self.accessory_pointers.read().await;
        let accessory = accessory_pointers.get(id).unwrap();
        let mut accessory = accessory.lock().await;
        if let Some(temperature) = state.temperature {
            let temperature_sensor_service = accessory
                .get_mut_service(HapType::TemperatureSensor)
                .unwrap();
            let current_temperature_characteristic = temperature_sensor_service
                .get_mut_characteristic(HapType::CurrentTemperature)
                .unwrap();
            current_temperature_characteristic
                .set_value(JsonValue::Number(
                    serde_json::Number::from_f64(temperature as f64).unwrap(),
                ))
                .await?;
        }
        if let Some(open_percent) = state.open_percent {
            let garage_door_opener_service = accessory
                .get_mut_service(HapType::GarageDoorOpener)
                .unwrap();
            let current_door_state_characteristic = garage_door_opener_service
                .get_mut_characteristic(HapType::CurrentDoorState)
                .unwrap();
            current_door_state_characteristic
                .set_value(JsonValue::Number(serde_json::Number::from(
                    if open_percent == 100 {
                        1
                    } else if open_percent == 0 {
                        0
                    } else {
                        unreachable!()
                    },
                )))
                .await?;
        }
        // TODO: Use other state fields
        Ok(())
    }

    async fn disconnected(&self, id: &accessory::ID) -> Result<(), Error> {
        let mut accessory_pointers = self.accessory_pointers.write().await;
        accessory_pointers.remove(&id);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "hap"
    }
}
