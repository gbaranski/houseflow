use async_trait::async_trait;
use auth::AuthCommand;
use run::RunCommand;

mod auth;
mod cli;
mod config;
mod run;

use config::{CliConfig, ClientConfig, Config, ConfigCommand, LogLevel, ServerConfig, Subcommand};
use strum_macros::{EnumIter, EnumString};

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

#[async_trait(?Send)]
pub trait ClientCommand {
    async fn run(&self, cfg: ClientConfig) -> anyhow::Result<()>;
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

fn setup_logging(log_level: &LogLevel) {
    use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};
    let level_filter: LevelFilter = log_level.clone().into();

    TermLogger::init(
        level_filter,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

fn main() -> anyhow::Result<()> {
    use clap::Clap;

    let cli_config = CliConfig::parse();
    actix_rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async {
        match cli_config.subcommand {
            Subcommand::Setup(cmd) => {
                setup_logging(&LogLevel::Info);
                cmd.run().await
            }
            Subcommand::Client(cmd) => {
                let config = config::read_files()?;
                setup_logging(&config.client.log_level);
                cmd.run(config.client).await
            }
            Subcommand::Server(cmd) => {
                let config = config::read_files()?;
                setup_logging(&config.server.log_level);
                cmd.run(config.server).await
            }
        }
    })
}
