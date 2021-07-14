use super::Device;
use async_trait::async_trait;
use houseflow_types::{DeviceCommand, DeviceError, DeviceStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecuteParams {
    NoOperation(()),
    OnOff { on: bool },
}

impl super::ExecuteParams for ExecuteParams {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Light {
    on: bool,
}

#[async_trait]
impl Device<ExecuteParams> for Light {
    async fn on_execute(
        &mut self,
        command: DeviceCommand,
        params: ExecuteParams,
    ) -> anyhow::Result<DeviceStatus> {
        let result = match command {
            DeviceCommand::NoOperation => DeviceStatus::Success,
            DeviceCommand::OnOff => match params {
                ExecuteParams::OnOff { on } => {
                    tracing::info!("setting light state to {}", on);
                    self.on = on;
                    DeviceStatus::Success
                }
                _ => DeviceStatus::Error(DeviceError::InvalidParameters),
            },
            _ => DeviceStatus::Error(DeviceError::FunctionNotSupported),
        };
        Ok(result)
    }

    fn state(&self) -> serde_json::Map<String, serde_json::Value> {
        serde_json::to_value(self)
            .unwrap()
            .as_object()
            .unwrap()
            .clone()
    }
}
