use super::DeviceCommandState;
use crate::Command;
use async_trait::async_trait;
use device::devices;
use types::DeviceType;

use clap::Clap;

#[derive(Clap)]
pub struct RunDeviceCommand {
    device_type: DeviceType,
}

#[async_trait(?Send)]
impl Command<DeviceCommandState> for RunDeviceCommand {
    async fn run(&self, state: DeviceCommandState) -> anyhow::Result<()> {
        log::info!(
            "Starting virtual device with ID: {}",
            state.config.device_id
        );

        async fn run_device<D: devices::Device<EP>, EP: devices::ExecuteParams>(
            state: DeviceCommandState,
            device: D,
        ) -> anyhow::Result<()> {
            device::run(state.config, device).await
        }

        match self.device_type {
            DeviceType::Light => run_device(state, devices::Light::default()),
            _ => unimplemented!(),
        }
        .await
    }
}
