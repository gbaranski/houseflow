use async_trait::async_trait;

mod cli;
mod config;

#[allow(unused_macros)]
macro_rules! cfg_if {
    ($($tt:tt)*) => { $($tt)* };
}

#[cfg(feature = "client")]
cfg_if! {
    mod auth;
    mod fulfillment;
    mod admin;

    pub use auth::AuthCommand;
    pub use fulfillment::FulfillmentCommand;
    pub use admin::AdminCommand;
    use houseflow_api::HouseflowAPI;
}

#[cfg(feature = "device")]
cfg_if! {
    mod device;

    pub use device::DeviceCommand;
}

#[cfg(feature = "server")]
cfg_if! {
    mod server;

    pub use server::ServerCommand;
}

pub use self::config::ConfigCommand;

use cli::{CliConfig, Subcommand};
use strum::{EnumIter, EnumString};

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

#[cfg(feature = "client")]
cfg_if! {
    use houseflow_config::client::Config as ClientConfig;
    use serde::{Deserialize, Serialize};
    use szafka::Szafka;
    use houseflow_types::{token::{AccessToken, RefreshToken}, Device};

    #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
    pub struct Tokens {
        access: String,
        refresh: String,
    }

    #[derive(Clone)]
    pub struct ClientCommandState {
        pub config: ClientConfig,
        pub houseflow_api: HouseflowAPI,
        pub tokens: Szafka<Tokens>,
        pub devices: Szafka<Vec<Device>>,
    }

    impl ClientCommandState {
        pub async fn new() -> anyhow::Result<Self> {
            use anyhow::Context;

            let config = ClientConfig::get(ClientConfig::default_path())
                .await
                .with_context(|| "read client config file")?;

            log::trace!("config loaded: {:#?}", config);
            let state = ClientCommandState {
                config: config.clone(),
                houseflow_api: HouseflowAPI::new(config.server_address),
                tokens: Szafka::new(houseflow_config::defaults::data_home().join("tokens")),
                devices: Szafka::new(houseflow_config::defaults::data_home().join("devices")),
            };
            Ok::<_, anyhow::Error>(state)
        }


        pub async fn access_token(&self) -> anyhow::Result<AccessToken> {
            use anyhow::Context;

            let tokens = self.tokens.get().await.with_context(|| "get tokens")?;
            let refresh_token = RefreshToken::decode_unsafe(&tokens.refresh).with_context(|| "you may need to log in again using `houseflow auth login`")?;

            let access_token = AccessToken::decode_unsafe(&tokens.access);
            match access_token {
                Ok(token) => {
                    log::debug!("cached access token is valid");
                    Ok(token)
                }
                Err(err) => {
                    log::debug!("token verify returned error: {}", err);
                    log::debug!("cached access token is expired, fetching new one");
                    let raw_fetched_access_token = self
                        .houseflow_api
                        .fetch_access_token(&refresh_token)
                        .await??.access_token;
                    let fetched_access_token = AccessToken::decode_unsafe(&raw_fetched_access_token)?;
                    let tokens = Tokens {
                        refresh: tokens.refresh,
                        access: raw_fetched_access_token,
                    };

                    self.tokens.save(&tokens).await?;
                    Ok(fetched_access_token)

                }
            }
        }

        pub async fn refresh_token(&self) -> anyhow::Result<RefreshToken> {
            use anyhow::Context;

            let tokens = self.tokens.get().await.with_context(|| "get tokens")?;
            RefreshToken::decode_unsafe(&tokens.refresh).with_context(|| "you may need to log in again using `houseflow auth login`")
        }
    }

}

#[cfg(feature = "server")]
cfg_if! {
    use houseflow_config::server::Config as ServerConfig;

    #[derive(Clone)]
    pub struct ServerCommandState {
        pub config: ServerConfig,
    }

    impl ServerCommandState {
        pub async fn new() -> anyhow::Result<Self> {
            use anyhow::Context;

            let config = ServerConfig::get(ServerConfig::default_path())
                .await
                .with_context(|| "read server config file")?;

            log::trace!("config loaded: {:#?}", config);
            let state = ServerCommandState { config };
            Ok::<_, anyhow::Error>(state)
        }
    }
}

#[cfg(feature = "device")]
cfg_if! {
    use houseflow_config::device::Config as DeviceConfig;

    #[derive(Clone)]
    pub struct DeviceCommandState {
        pub config: houseflow_config::device::Config,
    }

    impl DeviceCommandState {
        pub async fn new() -> anyhow::Result<Self> {
            use anyhow::Context;

            let config = DeviceConfig::get(DeviceConfig::default_path())
                .await
                .with_context(|| "read server config file")?;

            log::trace!("config loaded: {:#?}", config);
            let state = DeviceCommandState { config };
            Ok::<_, anyhow::Error>(state)
        }

    }

}

#[async_trait(?Send)]
pub trait Command<T: CommandState> {
    async fn run(self, state: T) -> anyhow::Result<()>;
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
        match cli_config.subcommand {
            #[cfg(feature = "client")]
            Subcommand::Auth(cmd) => cmd.run(ClientCommandState::new().await?).await,

            #[cfg(feature = "client")]
            Subcommand::Admin(cmd) => cmd.run(ClientCommandState::new().await?).await,

            #[cfg(feature = "client")]
            Subcommand::Fulfillment(cmd) => cmd.run(ClientCommandState::new().await?).await,

            #[cfg(feature = "server")]
            Subcommand::Server(cmd) => cmd.run(ServerCommandState::new().await?).await,

            #[cfg(feature = "device")]
            Subcommand::Device(cmd) => cmd.run(DeviceCommandState::new().await?).await,

            Subcommand::Config(cmd) => cmd.run(()).await,
        }
    })
}
