use super::ActorMessage;
use super::ControllerHandle;
use crate::ProviderHandle;
use futures::lock::Mutex;
use futures::FutureExt;
use hap::accessory::garage_door_opener::GarageDoorOpenerAccessory;
use hap::accessory::AccessoryCategory;
use hap::accessory::AccessoryInformation;
use hap::accessory::HapAccessory;
use hap::characteristic::AsyncCharacteristicCallbacks;
use hap::characteristic::CharacteristicCallbacks;
use hap::server::IpServer;
use hap::server::Server;
use hap::service::battery::BatteryService;
use hap::service::humidity_sensor::HumiditySensorService;
use hap::service::temperature_sensor::TemperatureSensorService;
use hap::storage::FileStorage;
use hap::storage::Storage;
use hap::HapType;
use hap::MacAddress;
use hap::Pin;
use houseflow_config::hub::manufacturers;
use houseflow_config::hub::AccessoryType;
use houseflow_config::hub::HapController as HapConfig;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::services::ServiceName;
use mac_address::get_mac_address;
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct HapController {
    receiver: mpsc::Receiver<ActorMessage>,
    ip_server: IpServer,
    provider: ProviderHandle,
    accessory_pointers: HashMap<accessory::ID, Arc<Mutex<Box<dyn HapAccessory>>>>,
    accessory_instance_id: u64,
}

impl HapController {
    pub async fn create(
        provider: ProviderHandle,
        config: HapConfig,
    ) -> Result<ControllerHandle, anyhow::Error> {
        let (tx, rx) = mpsc::channel(8);
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
        let ip_server = IpServer::new(config, storage).await?;
        let mut actor = HapController {
            receiver: rx,
            ip_server,
            provider,
            accessory_pointers: Default::default(),
            accessory_instance_id: 1,
        };
        let handle = ControllerHandle::new("hap", tx);
        tokio::spawn(async move { actor.run().await });
        Ok(handle)
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        let ip_server = self.ip_server.clone();
        tokio::spawn(async move {
            ip_server.run_handle().await.unwrap();
        });
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await?;
        }
        Ok(())
    }

    async fn handle_message(&mut self, message: ActorMessage) -> Result<(), anyhow::Error> {
        match message {
            ActorMessage::Connected {
                configured_accessory,
            } => {
                let accessory_ptr = match &configured_accessory.r#type {
                    AccessoryType::XiaomiMijia(accessory_type) => {
                        use manufacturers::XiaomiMijia as Manufacturer;

                        let manufacturer = "Xiaomi Mijia".to_string();
                        match accessory_type {
                            Manufacturer::HygroThermometer { mac_address: _ } => {
                                let mut hygro_thermometer = HygroThermometerAccessory {
                                    id: self.accessory_instance_id,
                                    accessory_information: AccessoryInformation {
                                        manufacturer,
                                        model: "LYWSD03MMC".to_string(), // TODO: ensure that this one is okay
                                        name: "HygroThermometer".to_string(),
                                        serial_number: configured_accessory.id.to_string(),
                                        ..Default::default()
                                    }
                                    .to_service(1, self.accessory_instance_id)
                                    .unwrap(),
                                    // accessory information service ends at IID 6, so we start counting at 7
                                    temperature_sensor: TemperatureSensorService::new(
                                        7,
                                        self.accessory_instance_id,
                                    ),
                                    // teperature sensor service ends at IID 13, so we start counting at 14
                                    humidity_sensor: HumiditySensorService::new(
                                        14,
                                        self.accessory_instance_id,
                                    ),
                                    // humidity sensor service ends at IID 20, so we start counting at 21
                                    battery: BatteryService::new(21, self.accessory_instance_id),
                                };
                                hygro_thermometer
                                    .temperature_sensor
                                    .current_temperature
                                    .on_read(Some(|| Ok(None)));

                                hygro_thermometer
                                    .humidity_sensor
                                    .current_relative_humidity
                                    .on_read(Some(|| Ok(None)));

                                hygro_thermometer
                                    .battery
                                    .battery_level
                                    .as_mut()
                                    .unwrap()
                                    .on_read(Some(|| Ok(None)));

                                hygro_thermometer
                                    .battery
                                    .status_low_battery
                                    .on_read(Some(|| Ok(None)));

                                hygro_thermometer
                                    .battery
                                    .charging_state
                                    .as_mut()
                                    .unwrap()
                                    .on_read(Some(|| Ok(Some(hap::characteristic::charging_state::Value::NotChargeable as u8))));

                                hygro_thermometer
                                    .battery
                                    .status_low_battery
                                    .on_read(Some(|| Ok(None)));

                                self.ip_server.add_accessory(hygro_thermometer).await?
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
                                    self.accessory_instance_id,
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

                                let provider = self.provider.clone();

                                let accessory_id = configured_accessory.id;
                                garage_door_opener
                                    .garage_door_opener
                                    .target_door_state
                                    .on_update_async(Some(move |current: u8, new: u8| {
                                        let provider = provider.clone();

                                        async move {
                                            println!("garage_door_opener target door state characteristic updated from {} to {}", current, new);
                                            let service_name = ServiceName::GarageDoorOpener;
                                            let characteristic = Characteristic::TargetDoorState(characteristics::TargetDoorState {
                                                open_percent: if new == 1 {
                                                                100
                                                            } else if new == 0 {
                                                                0
                                                            } else {
                                                                unreachable!()
                                                            },
                                            });

                                            provider.write_characteristic(accessory_id, service_name, characteristic).await.unwrap();
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
                self.accessory_instance_id += 1;
                self.accessory_pointers
                    .insert(configured_accessory.id, accessory_ptr);
            }
            ActorMessage::Disconnected { accessory_id } => {
                let accessory_pointer = self.accessory_pointers.remove(&accessory_id).unwrap();
                self.ip_server.remove_accessory(&accessory_pointer).await?;
            }
            ActorMessage::Updated {
                accessory_id,
                service_name,
                characteristic,
            } => {
                let accessory = self.accessory_pointers.get(&accessory_id).unwrap();
                let mut accessory = accessory.lock().await;
                let service_hap_type = match service_name {
                    ServiceName::TemperatureSensor => HapType::TemperatureSensor,
                    ServiceName::HumiditySensor => HapType::HumiditySensor,
                    ServiceName::GarageDoorOpener => HapType::GarageDoorOpener,
                    ServiceName::Battery => HapType::Battery,
                };
                let service = accessory.get_mut_service(service_hap_type).unwrap();
                match characteristic {
                    Characteristic::CurrentTemperature(current_temperature) => {
                        service
                            .get_mut_characteristic(HapType::CurrentTemperature)
                            .unwrap()
                            .set_value(JsonValue::Number(
                                serde_json::Number::from_f64(
                                    current_temperature.temperature as f64,
                                )
                                .unwrap(),
                            ))
                            .await?
                    }
                    Characteristic::CurrentHumidity(current_humidity) => {
                        service
                            .get_mut_characteristic(HapType::CurrentRelativeHumidity)
                            .unwrap()
                            .set_value(JsonValue::Number(
                                serde_json::Number::from_f64(current_humidity.humidity as f64)
                                    .unwrap(),
                            ))
                            .await?
                    }
                    Characteristic::CurrentDoorState(current_door_state) => {
                        service
                            .get_mut_characteristic(HapType::CurrentDoorState)
                            .unwrap()
                            .set_value(JsonValue::Number(serde_json::Number::from(
                                if current_door_state.open_percent == 100 {
                                    1
                                } else if current_door_state.open_percent == 0 {
                                    0
                                } else {
                                    unimplemented!()
                                },
                            )))
                            .await?
                    }
                    Characteristic::TargetDoorState(_) => unimplemented!(),
                    Characteristic::BatteryLevel(characteristics::BatteryLevel {
                        battery_level_percent,
                    }) => {
                        service
                            .get_mut_characteristic(HapType::BatteryLevel)
                            .unwrap()
                            .set_value(JsonValue::Number(serde_json::Number::from(
                                battery_level_percent,
                            )))
                            .await?;
                        service
                            .get_mut_characteristic(HapType::StatusLowBattery)
                            .unwrap()
                            .set_value(JsonValue::Number(serde_json::Number::from(
                                if battery_level_percent > 20 { 0 } else { 1 },
                            )))
                            .await?;
                    }
                    Characteristic::ChargingState(_) => todo!(),
                };
            }
        };
        Ok(())
    }
}

#[derive(Debug, Default)]
struct HygroThermometerAccessory {
    id: u64,

    pub accessory_information: hap::service::accessory_information::AccessoryInformationService,
    pub temperature_sensor: hap::service::temperature_sensor::TemperatureSensorService,
    pub humidity_sensor: hap::service::humidity_sensor::HumiditySensorService,
    pub battery: hap::service::battery::BatteryService,
}

impl hap::accessory::HapAccessory for HygroThermometerAccessory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id
    }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn hap::service::HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn hap::service::HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn hap::service::HapService> {
        vec![
            &self.accessory_information,
            &self.temperature_sensor,
            &self.humidity_sensor,
            &self.battery,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn hap::service::HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.temperature_sensor,
            &mut self.humidity_sensor,
            &mut self.battery,
        ]
    }
}

impl Serialize for HygroThermometerAccessory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
