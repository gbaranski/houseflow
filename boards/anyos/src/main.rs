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

    async fn on_command(&self, command: device::Command) -> anyhow::Result<device::Status> {
        tracing::info!(command = %command, "command received");
        // match command {
        //     device::Command::OnOff(commands::OnOff { on }) => todo!(),
        //     device::Command::OpenClose(commands::OpenClose { open_percent }) => todo!(),
        //     _ => todo!(),
        // };
        // tracing::info!("Changing `open_percent` to {0}", open_percent);
        // self.state.open_percent = open_percent;

        Ok(device::Status::Error(device::Error::FunctionNotSupported))
    }
}

#[tokio::main]
async fn main() {
    houseflow_config::init_logging(false);
    let config = Config::read(Config::default_path()).expect("cannot load device config");
    let server_config = config.server.clone();
    let device = Device {
        state: State { open_percent: 0 },
        config,
    };
    houseflow_device::run(server_config, device).await.unwrap();
}
