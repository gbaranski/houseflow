use async_trait::async_trait;
use houseflow_config::device::Config;
use houseflow_config::Config as _;
use houseflow_types::device;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct State {
    open_percent: u8,
    on: bool,
}

struct Device {
    state: State,
    config: Config,
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

    async fn on_command(&mut self, command: device::Command) -> anyhow::Result<device::Status> {
        tracing::info!(command = %command, "command received");
        let status = match command {
            device::Command::OnOff(params) => {
                tracing::info!("Set `on` to {}", params.on);
                self.state.on = params.on;
                device::Status::Success
            }
            device::Command::OpenClose(params) => {
                tracing::info!("Set `open_percent` to {}", params.open_percent);
                self.state.open_percent = params.open_percent;
                device::Status::Success
            }
            _ => device::Status::Error(device::Error::FunctionNotSupported),
        };
        Ok(status)
    }
}

#[tokio::main]
async fn main() {
    houseflow_config::init_logging(false);
    let config = Config::read(Config::default_path()).expect("cannot load device config");
    let server_config = config.server.clone();
    let device = Device {
        state: State {
            open_percent: 0,
            on: false,
        },
        config,
    };
    houseflow_device::run(server_config, device).await.unwrap();
}
