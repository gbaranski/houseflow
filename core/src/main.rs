use async_trait::async_trait;

mod auth;
mod cli;
mod config;
mod device;
mod fulfillment;
mod server;

pub use self::config::ConfigCommand;
pub use self::device::DeviceCommand;
pub use crate::{auth::AuthCommand, device::RunDeviceCommand, fulfillment::FulfillmentCommand};
pub use server::ServerCommand;

use anyhow::Context;
use cli::{CliConfig, Subcommand};
use houseflow_config::{
    client::Config as ClientConfig, defaults as ConfigDefaults, device::Config as DeviceConfig,
    server::Config as ServerConfig,
};
use houseflow_types::{token::Token, Device};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
use szafka::Szafka;

#[derive(Clone, Debug, EnumString, strum::Display, EnumIter)]
pub enum Target {
    #[strum(serialize = "server")]
    Server,

    #[strum(serialize = "client")]
    Client,

    #[strum(serialize = "device")]
    Device,
}

impl Target {
    pub fn config_path(&self) -> std::path::PathBuf {
        let base_path = xdg::BaseDirectories::with_prefix(clap::crate_name!())
            .unwrap()
            .get_config_home();
        match self {
            Target::Server => base_path.join("server.toml"),
            Target::Client => base_path.join("client.toml"),
            Target::Device => base_path.join("device.toml"),
        }
    }
}

pub trait CommandState {}

impl<T> CommandState for T {}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tokens {
    access: Token,
    refresh: Token,
}

#[derive(Clone)]
pub struct ClientCommandState {
    pub config: ClientConfig,
    pub houseflow_api: api::HouseflowAPI,
    pub tokens: Szafka<Tokens>,
    pub devices: Szafka<Vec<Device>>,
}

#[derive(Clone)]
pub struct ServerCommandState {
    pub config: ServerConfig,
}

#[derive(Clone)]
pub struct DeviceCommandState {
    pub config: houseflow_config::device::Config,
}

impl ClientCommandState {
    pub async fn access_token(&self) -> anyhow::Result<Token> {
        let tokens = self.tokens.get().await.with_context(|| "get tokens")?;
        if tokens.refresh.has_expired() {
            log::debug!("cached refresh token is expired");
            return Err(anyhow::Error::msg(
                "refresh token expired, you need to log in again using `houseflow auth login`",
            ));
        }

        if !tokens.access.has_expired() {
            log::debug!("cached access token is not expired");
            Ok(tokens.access)
        } else {
            log::debug!("cached access token is expired, fetching new one");
            let fetched_access_token = self
                .houseflow_api
                .fetch_access_token(&tokens.refresh)
                .await?
                .into_result()?
                .access_token;
            let tokens = Tokens {
                refresh: tokens.refresh,
                access: tokens.access,
            };
            self.tokens.save(&tokens).await?;

            Ok(fetched_access_token)
        }
    }
}

#[async_trait(?Send)]
pub trait Command<T: CommandState> {
    async fn run(&self, state: T) -> anyhow::Result<()>;
}

fn main() -> anyhow::Result<()> {
    use clap::Clap;

    env_logger::init_from_env(env_logger::Env::default().filter_or("HOUSEFLOW_LOG", "info"));

    let cli_config = CliConfig::parse();
    actix_rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async {
        let client_command_state = || async {
            let config = ClientConfig::get(ClientConfig::default_path())
                .await
                .with_context(|| "read client config file")?;

            log::trace!("config loaded: {:#?}", config);
            let state = ClientCommandState {
                config: config.clone(),
                houseflow_api: api::HouseflowAPI::new(config.server_address),
                tokens: Szafka::new(ConfigDefaults::data_home().join("tokens")),
                devices: Szafka::new(ConfigDefaults::data_home().join("devices")),
            };
            Ok::<_, anyhow::Error>(state)
        };

        let server_command_state = || async {
            let config = ServerConfig::get(ServerConfig::default_path())
                .await
                .with_context(|| "read server config file")?;

            log::trace!("config loaded: {:#?}", config);
            let state = ServerCommandState { config };
            Ok::<_, anyhow::Error>(state)
        };

        let device_command_state = || async {
            let config = DeviceConfig::get(DeviceConfig::default_path())
                .await
                .with_context(|| "read server config file")?;

            log::trace!("config loaded: {:#?}", config);
            let state = DeviceCommandState { config };
            Ok::<_, anyhow::Error>(state)
        };

        match cli_config.subcommand {
            Subcommand::Auth(cmd) => cmd.run(client_command_state().await?).await,
            Subcommand::Fulfillment(cmd) => cmd.run(client_command_state().await?).await,
            Subcommand::Server(cmd) => cmd.run(server_command_state().await?).await,
            Subcommand::Device(cmd) => cmd.run(device_command_state().await?).await,
            Subcommand::Config(cmd) => cmd.run(()).await,
        }
    })
}
