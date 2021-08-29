use async_trait::async_trait;
use houseflow_config::device::Credentials;
use houseflow_config::device::Server;
use houseflow_types::DeviceError;
use houseflow_types::DeviceStatus;
use session::Session;

mod session;

pub async fn run(server_config: Server, device: impl Device) -> anyhow::Result<()> {
    let session = Session::new(server_config);
    session.run(device).await?;

    Ok(())
}

#[async_trait]
pub trait Device
where
    Self: Send,
{
    fn state(&self) -> anyhow::Result<serde_json::Map<String, serde_json::Value>>;
    fn credentials(&self) -> &Credentials;

    #[allow(unused_variables)]
    async fn on_off(&mut self, on: bool) -> anyhow::Result<DeviceStatus> {
        Ok(DeviceStatus::Error(DeviceError::FunctionNotSupported))
    }

    #[allow(unused_variables)]
    async fn open_close(&mut self, open_percent: u8) -> anyhow::Result<DeviceStatus> {
        Ok(DeviceStatus::Error(DeviceError::FunctionNotSupported))
    }
}
