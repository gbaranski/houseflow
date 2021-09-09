use async_trait::async_trait;
use houseflow_config::device::Credentials;
use houseflow_config::device::Server;
use houseflow_types::device;
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

    async fn on_command(&self, command: device::Command) -> anyhow::Result<device::Status>;
}
