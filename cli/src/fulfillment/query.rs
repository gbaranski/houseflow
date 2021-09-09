use crate::CommandContext;
use async_trait::async_trait;
use houseflow_types::fulfillment::query;
use houseflow_types::lighthouse;
use houseflow_types::device;

pub struct Command {
    pub device_id: device::ID,
}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let access_token = ctx.access_token().await?;
        let devices = ctx.devices.get().await?;
        let _ = devices
            .iter()
            .find(|device| device.id == self.device_id)
            .ok_or_else(|| {
                anyhow::Error::msg(
                    "device not found, try `houseflow fulfillment sync` to fetch new devices",
                )
            })?;

        let query_frame = lighthouse::query::Frame {};
        let request = query::Request {
            device_id: self.device_id.clone(),
            frame: query_frame,
        };
        let response = ctx
            .houseflow_api()
            .await?
            .query(&access_token, &request)
            .await??;

        println!("Device responded with state: {:#?}", response.frame.state);

        Ok(())
    }
}
