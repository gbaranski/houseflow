use async_trait::async_trait;
use houseflow_types::accessory;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

#[async_trait]
pub trait Accessory {
    async fn execute(&self, command: accessory::Command) -> Result<accessory::Status, Error>;
    async fn state(&self) -> Result<accessory::State, Error>;
}
