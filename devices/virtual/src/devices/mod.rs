mod light;

pub use light::Light;

use async_trait::async_trait;
use houseflow_types::{DeviceCommand, DeviceStatus};

pub trait ExecuteParams: serde::de::DeserializeOwned {}

#[async_trait]
pub trait Device<EP>
where
    EP: ExecuteParams,
{
    async fn on_execute(
        &mut self,
        command: DeviceCommand,
        params: EP,
    ) -> anyhow::Result<DeviceStatus>;

    fn state(&self) -> serde_json::Map<String, serde_json::Value>;
}
