use async_trait::async_trait;
use houseflow_accessory_hal::Accessory;
use houseflow_accessory_hal::AccessoryEvent;
use houseflow_accessory_hal::AccessoryEventSender;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::services::Service;
use houseflow_types::accessory::services::ServiceDiscriminants;
use houseflow_types::accessory::Error;
use houseflow_types::accessory::{characteristics, services};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::Mutex;

pub struct VirtualAccessory {
    services: Mutex<HashMap<ServiceDiscriminants, Service>>,
    events: AccessoryEventSender,
}

impl VirtualAccessory {
    pub fn new(events: AccessoryEventSender) -> Self {
        let mut services = HashMap::new();
        services.insert(
            ServiceDiscriminants::GarageDoorOpener,
            Service::GarageDoorOpener(services::GarageDoorOpener {
                current_door_state: characteristics::CurrentDoorState { open_percent: 100 },
                target_door_state: characteristics::TargetDoorState { open_percent: 100 },
            }),
        );

        Self {
            services: Mutex::new(services),
            events,
        }
    }
}

#[async_trait]
impl Accessory for VirtualAccessory {
    async fn write_characteristic(
        &self,
        service_name: ServiceDiscriminants,
        characteristic: Characteristic,
    ) -> Result<(), Error> {
        let mut services = self.services.lock().await;
        match services.get_mut(&service_name) {
            Some(service) => match (service, &characteristic) {
                (
                    Service::GarageDoorOpener(service),
                    Characteristic::TargetDoorState(characteristics::TargetDoorState {
                        open_percent,
                    }),
                ) => {
                    service.target_door_state.open_percent = *open_percent;
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    service.current_door_state.open_percent = *open_percent;
                    self.events
                        .send(AccessoryEvent::CharacteristicUpdate {
                            service_name,
                            characteristic: Characteristic::CurrentDoorState(
                                service.current_door_state.to_owned(),
                            ),
                        })
                        .unwrap();
                }
                _ => return Err(accessory::Error::CharacteristicNotSupported),
            },
            None => return Err(accessory::Error::ServiceNotSupported),
        };

        Ok(())
    }

    async fn read_characteristic(
        &self,
        service_name: ServiceDiscriminants,
        characteristic_name: characteristics::CharacteristicDiscriminants,
    ) -> Result<Characteristic, Error> {
        let services = self.services.lock().await;
        match services.get(&service_name) {
            Some(service) => match (service, characteristic_name) {
                (
                    Service::TemperatureSensor(sensor),
                    characteristics::CharacteristicDiscriminants::CurrentTemperature,
                ) => Ok(Characteristic::CurrentTemperature(
                    sensor.current_temperature.clone(),
                )),
                (
                    Service::HumiditySensor(sensor),
                    characteristics::CharacteristicDiscriminants::CurrentHumidity,
                ) => Ok(Characteristic::CurrentHumidity(
                    sensor.current_humidity.clone(),
                )),
                (
                    Service::GarageDoorOpener(sensor),
                    characteristics::CharacteristicDiscriminants::CurrentDoorState,
                ) => Ok(Characteristic::CurrentDoorState(
                    sensor.current_door_state.clone(),
                )),
                (
                    Service::GarageDoorOpener(sensor),
                    characteristics::CharacteristicDiscriminants::TargetDoorState,
                ) => Ok(Characteristic::TargetDoorState(
                    sensor.target_door_state.clone(),
                )),
                _ => Err(Error::CharacteristicNotSupported),
            },
            None => Err(Error::ServiceNotSupported),
        }
    }
}
