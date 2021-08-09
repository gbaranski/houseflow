use async_trait::async_trait;
use houseflow_config::{
    device::{Config, Credentials, Light},
    Config as _,
};
use houseflow_types::DeviceStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct State {
    on: bool,
}

struct Device {
    state: State,
    config: Light,
}

#[async_trait]
impl houseflow_device::Device for Device {
    fn state(&self) -> anyhow::Result<serde_json::Map<String, serde_json::Value>> {
        Ok(serde_json::to_value(&self.state)?
            .as_object()
            .unwrap()
            .to_owned())
    }

    async fn on_off(&mut self, on: bool) -> anyhow::Result<DeviceStatus> {
        tracing::info!("Changing `on` to {0}", on);
        self.state.on = on;

        Ok(DeviceStatus::Success)
    }

    fn credentials(&self) -> &Credentials {
        &self.config.credentials
    }
}

#[tokio::main]
async fn main() {
    houseflow_config::init_logging(false);
    let path = Config::default_path();
    tracing::debug!("Config path: {}", path.to_str().unwrap());
    let config = if path.exists() {
        Config::read(path).expect("cannot load device config")
    } else {
        Config::default()
    };
    let device_config = config.light.expect("light is not configured");
    let device = Device {
        state: State { on: false },
        config: device_config,
    };

    houseflow_device::run(config.server, device).await.unwrap();
}
