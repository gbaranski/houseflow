use async_trait::async_trait;
use houseflow_accessory_hal::Accessory;
use houseflow_api::hub::hive::HiveClient;
use houseflow_config::accessory::Services;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::characteristics::CurrentTemperature;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::accessory::Error;
use std::str::FromStr;

pub struct VirtualAccessory {
    services: Services,
}

impl VirtualAccessory {
    pub fn new(client: HiveClient<VirtualAccessory>, services: Services) -> Self {
        let Services { temperature_sensor } = services.clone();
        if let Some(service) = temperature_sensor {
            tokio::spawn(async move {
                loop {
                    let temperature = service.current_temperature.command.execute().unwrap();
                    let temperature = std::str::from_utf8(&temperature).unwrap().trim();
                    let temperature = f32::from_str(temperature).unwrap();
                    let characteristic =
                        Characteristic::CurrentTemperature(CurrentTemperature { temperature });
                    client
                        .update(ServiceName::TemperatureSensor, characteristic)
                        .await;
                    tokio::time::sleep(service.current_temperature.interval).await;
                }
            });
        };
        Self { services }
    }
}

#[async_trait]
impl Accessory for VirtualAccessory {
    async fn write_characteristic(
        &mut self,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), Error> {
        match service_name {
            ServiceName::TemperatureSensor if self.services.temperature_sensor.is_some() => {
                match characteristic {
                    Characteristic::CurrentTemperature(_) => {
                        return Err(accessory::Error::CharacteristicReadOnly)
                    }
                    _ => return Err(accessory::Error::CharacteristicNotSupported),
                }
            }
            _ => return Err(accessory::Error::ServiceNotSupported),
        };
    }

    async fn read_characteristic(
        &mut self,
        service_name: ServiceName,
        characteristic_name: characteristics::CharacteristicName,
    ) -> Result<Characteristic, Error> {
        match service_name {
            ServiceName::TemperatureSensor if self.services.temperature_sensor.is_some() => {
                let service = self.services.temperature_sensor.as_ref().unwrap();
                match characteristic_name {
                    CharacteristicName::CurrentTemperature => {
                        let temperature = service.current_temperature.command.execute().unwrap();
                        let temperature = std::str::from_utf8(&temperature).unwrap().trim();
                        let temperature = f32::from_str(temperature).unwrap();
                        Ok(Characteristic::CurrentTemperature(CurrentTemperature {
                            temperature,
                        }))
                    }
                    _ => return Err(accessory::Error::CharacteristicNotSupported),
                }
            }
            _ => return Err(accessory::Error::ServiceNotSupported),
        }
    }
}
