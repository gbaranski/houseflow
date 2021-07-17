use async_trait::async_trait;
use houseflow_config::device::Config;
use houseflow_types::{DeviceError, DeviceStatus};
use session::Session;

mod session;

pub async fn run(
    cfg: Config,
    device: impl Device,
) -> anyhow::Result<()> {
    let session = Session::new(cfg);
    session.run(device).await?;

    Ok(())
}

#[async_trait]
pub trait Device
where
    Self: Send,
{
    fn state(&self) -> anyhow::Result<serde_json::Map<String, serde_json::Value>>;

    #[allow(unused_variables)]
    async fn on_off(&mut self, on: bool) -> anyhow::Result<DeviceStatus> {
        Ok(DeviceStatus::Error(DeviceError::FunctionNotSupported))
    }

    #[allow(unused_variables)]
    async fn open_close(&mut self, open_percent: u8) -> anyhow::Result<DeviceStatus> {
        Ok(DeviceStatus::Error(DeviceError::FunctionNotSupported))
    }
}
