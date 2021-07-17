use async_trait::async_trait;
use houseflow_config::device::Config;
use houseflow_types::DeviceStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct State {
    on: bool,
}

struct Device {
    state: State,
}

#[async_trait]
impl houseflow_device::Device for Device {
    async fn on_off(&mut self, on: bool) -> anyhow::Result<DeviceStatus> {
        tracing::info!("Changing `on` to {0}", on);
        self.state.on = on;

        Ok(DeviceStatus::Success)
    }

    fn state(&self) -> anyhow::Result<serde_json::Map<String, serde_json::Value>> {
        Ok(serde_json::to_value(&self.state)?
            .as_object()
            .unwrap()
            .to_owned())
    }
}

#[tokio::main]
async fn main() {
    houseflow_config::init_logging();
    let config = Config::get(Config::default_path())
        .await
        .expect("cannot load device config");
    let device = Device {
        state: State { on: false },
    };
    houseflow_device::run(config, device).await.unwrap();
}
