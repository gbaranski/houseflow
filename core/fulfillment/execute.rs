use crate::{ClientCommandState, Command};
use async_trait::async_trait;
use houseflow_types::{
    fulfillment::execute, lighthouse::proto, DeviceCommand, DeviceID, DeviceStatus,
};

use clap::Clap;

fn object_from_str(s: &str) -> Result<serde_json::Map<String, serde_json::Value>, serde_json::Error> {
    serde_json::from_str(s)
}

#[derive(Clap)]
pub struct ExecuteCommand {
    pub device_id: DeviceID,
    pub command: DeviceCommand,

    #[clap(default_value = "{}", parse(try_from_str = object_from_str))]
    pub params: serde_json::Map<String, serde_json::Value>,
}

#[async_trait(?Send)]
impl Command<ClientCommandState> for ExecuteCommand {
    async fn run(self, state: ClientCommandState) -> anyhow::Result<()> {
        let access_token = state.access_token().await?;
        let devices = state.devices.get().await?;
        let device = devices
            .iter()
            .find(|device| device.id == self.device_id)
            .ok_or_else(|| {
                anyhow::Error::msg(
                    "device not found, try `houseflow fulfillment sync` to fetch new devices",
                )
            })?;
        let supported_commands = device
            .traits
            .iter()
            .flat_map(|device_trait| device_trait.commands());
        let is_supported = supported_commands
            .clone()
            .any(|command| command == self.command);

        if !is_supported {
            return Err(anyhow::Error::msg(format!(
                "command is not supported by the device\nsupported commands: {}",
                supported_commands
                    .map(|command| command.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )));
        }

        let execute_frame = proto::execute::Frame {
            id: rand::random(),
            command: self.command.clone(),
            params: self.params.clone(),
        };
        let request = execute::Request {
            device_id: self.device_id.clone(),
            frame: execute_frame,
        };
        let response = state
            .houseflow_api
            .execute(&access_token, &request)
            .await??;
        match response.frame.status {
            DeviceStatus::Success => println!("✔ Device responded with success!"),
            DeviceStatus::Error => println!(
                "❌ Device responded with error! Error: {}",
                response.frame.error
            ),
        }

        Ok(())
    }
}
