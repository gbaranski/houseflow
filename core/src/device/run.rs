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
            device::run(
                device::Config {
                    device_id: state.config.device_id,
                    device_password: state.config.device_password,
                    lighthouse_url: state.config.base_url.join("lighthouse/").unwrap(),
                },
                device,
            )
            .await
        }

        match self.device_type {
            DeviceType::Light => run_device(state, devices::Light::default()),
            _ => unimplemented!(),
        }
        .await
    }
}
