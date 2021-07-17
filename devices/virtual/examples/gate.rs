use async_trait::async_trait;
use houseflow_config::device::Config;
use houseflow_types::DeviceStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct State {
    open_percent: u8,
}

struct Device {
    state: State,
}

#[async_trait]
impl houseflow_device::Device for Device {
    fn state(&self) -> anyhow::Result<serde_json::Map<String, serde_json::Value>> {
        Ok(serde_json::to_value(&self.state)?
            .as_object()
            .unwrap()
            .to_owned())
    }

    async fn open_close(&mut self, open_percent: u8) -> anyhow::Result<DeviceStatus> {
        tracing::info!("Changing `open_percent` to {0}", open_percent);
        self.state.open_percent = open_percent;

        Ok(DeviceStatus::Success)
    }
}

#[tokio::main]
async fn main() {
    houseflow_config::init_logging();
    let config = Config::get(Config::default_path())
        .await
        .expect("cannot load device config");
    let device = Device {
        state: State { open_percent: 0 },
    };
    houseflow_device::run(config, device).await.unwrap();
}
