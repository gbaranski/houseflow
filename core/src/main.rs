use async_trait::async_trait;

mod auth;
mod cli;
mod config;
mod device;
mod fulfillment;
mod keystore;
mod server;

pub use self::device::DeviceCommand;
pub use crate::{auth::AuthCommand, device::RunDeviceCommand, fulfillment::FulfillmentCommand};
pub use config::ConfigCommand;
pub use keystore::{Keystore, KeystoreFile};
pub use server::ServerCommand;

use ::auth::api::Auth as AuthAPI;
use ::fulfillment::api::Fulfillment as FulfillmentAPI;
use cli::{CliConfig, Subcommand};
use config::{ClientConfig, DeviceConfig, ServerConfig};
use strum_macros::{EnumIter, EnumString};
use token::Token;

#[derive(Clone, Debug, EnumString, strum_macros::Display, EnumIter)]
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

#[derive(Clone)]
pub struct ClientCommandState {
    pub config: ClientConfig,
    pub keystore: Keystore,
    pub auth: AuthAPI,
    pub fulfillment: FulfillmentAPI,
}

#[derive(Clone)]
pub struct ServerCommandState {
    pub config: ServerConfig,
}

#[derive(Clone)]
pub struct DeviceCommandState {
    pub config: DeviceConfig,
}

impl ClientCommandState {
    pub async fn access_token(&self) -> anyhow::Result<Token> {
        let keystore_file = self.keystore.read().await?;
        if keystore_file.refresh_token.has_expired() {
            log::debug!("cached refresh token is expired");
            return Err(anyhow::Error::msg(
                "refresh token expired, you need to log in again using `houseflow auth login`",
            ));
        }

        if !keystore_file.access_token.has_expired() {
            log::debug!("cached access token is not expired");
            Ok(keystore_file.access_token)
        } else {
            log::debug!("cached access token is expired, fetching new one");
            let fetched_access_token = self
                .auth
                .fetch_access_token(&keystore_file.refresh_token)
                .await?
                .into_result()?
                .access_token;
            let keystore_file = KeystoreFile {
                refresh_token: keystore_file.refresh_token,
                access_token: fetched_access_token.clone(),
            };
            self.keystore.save(&keystore_file).await?;

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
        use config::read_config_file;

        let client_command_state = || async {
            use anyhow::Context;
            let config: ClientConfig =
                read_config_file::<ClientConfig>(&Target::Client.config_path())
                    .await
                    .with_context(|| "read client config file")?;
            let state = ClientCommandState {
                config: config.clone(),
                auth: AuthAPI::new(config.base_url.join("auth").unwrap()),
                fulfillment: FulfillmentAPI::new(config.base_url.join("fulfillment").unwrap()),
                keystore: Keystore {
                    path: config.keystore_path,
                },
            };
            Ok::<_, anyhow::Error>(state)
        };

        let server_command_state = || async {
            use anyhow::Context;
            let config: ServerConfig =
                read_config_file::<ServerConfig>(&Target::Server.config_path())
                    .await
                    .with_context(|| "read server config file")?;
            let state = ServerCommandState { config };
            Ok::<_, anyhow::Error>(state)
        };

        let device_command_state = || async {
            use anyhow::Context;

            let config: DeviceConfig =
                read_config_file::<DeviceConfig>(&Target::Device.config_path())
                    .await
                    .with_context(|| "read device config file")?;
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
