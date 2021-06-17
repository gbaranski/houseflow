use async_trait::async_trait;

mod auth;
mod cli;
mod config;
mod fulfillment;
mod keystore;
mod run;

pub use crate::auth::AuthCommand;
pub use crate::fulfillment::FulfillmentCommand;
use ::auth::api::Auth as AuthAPI;
use ::fulfillment::api::Fulfillment as FulfillmentAPI;
pub use config::ConfigCommand;
pub use keystore::{Keystore, KeystoreFile};
pub use run::RunCommand;

use cli::{CliConfig, Subcommand};
use config::{ClientConfig, Config, ServerConfig};
use strum_macros::{EnumIter, EnumString};
use token::Token;

#[derive(Clone, Debug, EnumString, strum_macros::Display, EnumIter)]
pub enum Target {
    Server,
    Client,
}

impl Target {
    pub fn config_path(&self) -> std::path::PathBuf {
        let base_path = xdg::BaseDirectories::with_prefix(clap::crate_name!())
            .unwrap()
            .get_config_home();
        match self {
            Target::Server => base_path.join("server.toml"),
            Target::Client => base_path.join("client.toml"),
        }
    }
}

#[derive(Clone)]
pub struct ClientCommandState {
    pub config: ClientConfig,
    pub keystore: Keystore,
    pub auth: AuthAPI,
    pub fulfillment: FulfillmentAPI,
}

impl ClientCommandState {
    pub async fn access_token(&self) -> anyhow::Result<Token> {
        let keystore_file = self.keystore.read().await?;
        if keystore_file.refresh_token.has_expired() == true {
            log::debug!("cached refresh token is expired");
            return Err(anyhow::Error::msg(
                "refresh token expired, you need to log in again using `houseflow auth login`",
            ));
        }

        if keystore_file.access_token.has_expired() == false {
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
pub trait ClientCommand {
    async fn run(&self, state: ClientCommandState) -> anyhow::Result<()>;
}

#[async_trait(?Send)]
pub trait ServerCommand {
    async fn run(&self, cfg: ServerConfig) -> anyhow::Result<()>;
}

#[async_trait(?Send)]
pub trait Command {
    async fn run(&self, cfg: Config) -> anyhow::Result<()>;
}

// Consider changing name here
#[async_trait(?Send)]
pub trait SetupCommand {
    async fn run(&self) -> anyhow::Result<()>;
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
            Subcommand::Setup(cmd) => cmd.run().await,
            Subcommand::Client(cmd) => {
                let config = config::read_files()?.client;
                let keystore = Keystore {
                    path: config.keystore_path.clone(),
                };
                let auth = AuthAPI {
                    url: config.auth_url.clone(),
                };
                let fulfillment = FulfillmentAPI {
                    url: config.fulfillment_url.clone(),
                };
                let state = ClientCommandState {
                    config,
                    keystore,
                    auth,
                    fulfillment,
                };
                cmd.run(state).await
            }
            Subcommand::Server(cmd) => {
                let config = config::read_files()?;
                cmd.run(config.server).await
            }
        }
    })
}
