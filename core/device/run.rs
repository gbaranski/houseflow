use super::DeviceCommandState;
use crate::Command;
use async_trait::async_trait;
use houseflow_device::devices;
use houseflow_types::DeviceType;

use clap::Clap;

#[derive(Clap)]
pub struct RunDeviceCommand {
    device_type: DeviceType,
}

#[async_trait(?Send)]
impl Command<DeviceCommandState> for RunDeviceCommand {
    async fn run(self, state: DeviceCommandState) -> anyhow::Result<()> {
        tracing::info!(
            "Starting virtual device with ID: {}",
            state.config.device_id
        );

        async fn run_device<D: devices::Device<EP>, EP: devices::ExecuteParams>(
            state: DeviceCommandState,
            device: D,
        ) -> anyhow::Result<()> {
            houseflow_device::run(state.config, device).await
        }

        match self.device_type {
            DeviceType::Light => run_device(state, devices::light::Device::default()),
            _ => unimplemented!(),
        }
        .await
    }
}
