mod run;

pub use run::RunDeviceCommand;
use crate::DeviceConfig;

pub struct DeviceCommandState {
    pub config: DeviceConfig,
}

