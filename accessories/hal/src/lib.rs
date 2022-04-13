use accessory::Error;
use async_trait::async_trait;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;

#[async_trait]
pub trait Accessory: Send + Sync + 'static {
    async fn write_characteristic(
        &mut self,
        service_name: ServiceName,
        characteristic: Characteristic,
    ) -> Result<(), Error>;

    async fn read_characteristic(
        &mut self,
        service_name: ServiceName,
        characteristic_name: CharacteristicName,
    ) -> Result<Characteristic, Error>;
}
