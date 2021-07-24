use async_trait::async_trait;
use houseflow_config::{
    device::{Config, Gate},
    Config as _,
};
use houseflow_types::DeviceStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct State {
    open_percent: u8,
}

struct Device {
    state: State,
    config: Gate,
}

#[async_trait]
impl houseflow_device::Device for Device {
    fn state(&self) -> anyhow::Result<serde_json::Map<String, serde_json::Value>> {
        Ok(serde_json::to_value(&self.state)?
            .as_object()
            .unwrap()
            .to_owned())
    }

    fn credentials(&self) -> &houseflow_config::device::Credentials {
        &self.config.credentials
    }

    async fn open_close(&mut self, open_percent: u8) -> anyhow::Result<DeviceStatus> {
        tracing::info!("Changing `open_percent` to {0}", open_percent);
        self.state.open_percent = open_percent;

        Ok(DeviceStatus::Success)
    }
}

#[tokio::main]
async fn main() {
    houseflow_config::init_logging(true);
    let config = Config::read(Config::default_path()).expect("cannot load device config");
    let device_config = config.gate.expect("gate is not configured");
    let device = Device {
        state: State { open_percent: 0 },
        config: device_config,
    };
    houseflow_device::run(config.server, device).await.unwrap();
}
