use crate::CommandContext;
use houseflow_types::device;
use houseflow_types::fulfillment::query;
use houseflow_types::lighthouse;

pub struct Command {
    pub device_id: device::ID,
}

impl crate::Command for Command {
    fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let access_token = ctx.access_token()?;
        let devices = ctx.devices.get()?;
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
            device_id: self.device_id,
            frame: query_frame,
        };
        let response = ctx.client()?.query(&access_token, &request)??;

        println!("Device responded with state: {:#?}", response.frame.state);

        Ok(())
    }
}
