use std::time::Instant;

use crate::CommandContext;
use async_trait::async_trait;
use houseflow_types::device;
use houseflow_types::fulfillment::execute;
use houseflow_types::lighthouse;

pub struct Command {
    pub device_id: device::ID,
    pub command: device::Command,
    pub params: serde_json::Map<String, serde_json::Value>,
}

#[async_trait]
impl crate::Command for Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()> {
        let access_token = ctx.access_token().await?;
        let devices = match ctx.devices.get().await {
            Ok(devices) => devices,
            Err(szafka::Error::OpenFileError(err)) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(anyhow::Error::msg(
                        "no cached devices found, try `houseflow fulfillment sync`",
                    ))
                }
                _ => return Err(err.into()),
            },
            Err(err) => return Err(err.into()),
        };
        let device = devices
            .iter()
            .find(|device| device.id == self.device_id)
            .ok_or_else(|| {
                anyhow::Error::msg(
                    "device not found, try `houseflow fulfillment sync` to fetch new devices",
                )
            })?;
        if !self.command.is_supported(&device.traits) {
            return Err(anyhow::Error::msg("Command is not supported by the device"));
        }
        let execute_frame = lighthouse::execute::Frame {
            id: rand::random(),
            command: self.command.clone(),
        };
        let request = execute::Request {
            device_id: self.device_id.clone(),
            frame: execute_frame,
        };

        let before = Instant::now();
        let response = ctx
            .houseflow_api()
            .await?
            .execute(&access_token, &request)
            .await??;

        match response.frame.status {
            device::Status::Success => {
                println!(
                    "✔ Device responded with success after {}ms!",
                    Instant::now().duration_since(before).as_millis()
                )
            }
            device::Status::Error(err) => {
                println!("❌ Device responded with error! Error: {}", err)
            }
        };

        Ok(())
    }
}
