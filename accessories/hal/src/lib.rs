use async_trait::async_trait;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicDiscriminants;
use houseflow_types::accessory::services::ServiceDiscriminants;
use accessory::Error;
use tokio::sync::mpsc;


#[derive(Debug, Clone)]
pub enum AccessoryEvent {
    CharacteristicUpdate{
        service_name: ServiceDiscriminants,
        characteristic: Characteristic,
    }
}

pub type AccessoryEventSender = mpsc::UnboundedSender<AccessoryEvent>;
pub type AccessoryEventReceiver = mpsc::UnboundedReceiver<AccessoryEvent>;

#[async_trait]
pub trait Accessory {
    async fn write_characteristic(
        &self,
        service_name: ServiceDiscriminants,
        characteristic: Characteristic,
    ) -> Result<(), Error>;

    async fn read_characteristic(
        &self,
        service_name: ServiceDiscriminants,
        characteristic_name: CharacteristicDiscriminants,
    ) -> Result<Characteristic, Error>;
}
