use houseflow_config::hub::manufacturers;
pub use houseflow_config::hub::HapProvider as HapConfig;

use super::AdditionalAccessoryInfo;
use super::Service;
use crate::AccessoryState;
use anyhow::Error;
use async_trait::async_trait;
use futures::lock::Mutex;
use hap::accessory::temperature_sensor::TemperatureSensorAccessory;
use hap::accessory::AccessoryCategory;
use hap::accessory::AccessoryInformation;
use hap::accessory::HapAccessory;
use hap::server::IpServer;
use hap::server::Server;
use hap::storage::FileStorage;
use hap::storage::Storage;
use hap::HapType;
use hap::MacAddress;
use hap::Pin;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::AccessoryType;
use houseflow_types::accessory;
use mac_address::get_mac_address;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct HapService {
    ip_server: IpServer,
    accessory_pointers: RwLock<HashMap<accessory::ID, Arc<Mutex<Box<dyn HapAccessory>>>>>,
}

impl HapService {
    pub async fn new(config: &HapConfig) -> Result<Self, Error> {
        let mut storage = FileStorage::current_dir().await?;
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
        })
    }
}

#[async_trait]
impl Service for HapService {
    async fn run(&self) -> Result<(), Error> {
        self.ip_server.run_handle().await?;
        Ok(())
    }

    async fn connected(
        &self,
        configured_accessory: &Accessory,
        _additional_accessory_info: &AdditionalAccessoryInfo,
    ) -> Result<(), Error> {
        let accessory = match &configured_accessory.r#type {
            AccessoryType::XiaomiMijia(manufacturers::XiaomiMijia::HygroThermometer {
                mac_address,
            }) => {
                let temperature_sensor = TemperatureSensorAccessory::new(
                    uuid_to_u64(&configured_accessory.id),
                    AccessoryInformation {
                        manufacturer: String::from("Xiaomi"),
                        model: String::from("Temperature Sensor"),
                        name: String::from("Mijia Thermometer"),
                        serial_number: mac_address.to_owned(),
                        accessory_flags: None,
                        application_matching_identifier: None,
                        configured_name: Some(configured_accessory.name.clone()),
                        firmware_revision: None,
                        hardware_finish: None,
                        hardware_revision: None,
                        product_data: None,
                        software_revision: None,
                    },
                )?;

                temperature_sensor
            }
            _ => unimplemented!(),
        };
        let accessory_ptr = self.ip_server.add_accessory(accessory).await?;
        let mut accessory_pointers = self.accessory_pointers.write().await;
        accessory_pointers.insert(configured_accessory.id, accessory_ptr);
        Ok(())
    }

    async fn update_state(&self, id: &accessory::ID, state: &AccessoryState) -> Result<(), Error> {
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

pub fn uuid_to_u64(id: &uuid::Uuid) -> u64 {
    (id.as_u128() % u64::max_value() as u128) as u64
}
