use async_trait::async_trait;
use auth::AuthCommand;
use run::RunCommand;
use structopt::StructOpt;

mod auth;
mod cli;
mod config;
mod run;

use crate::config::{CliConfig, ClientConfig, Command, Config, LogLevel, ServerConfig};

// #[async_trait(?Send)]
// impl Command for RootCommand {
//     async fn run(&self, cfg: &Config) -> anyhow::Result<()> {
//         match self {
//             Self::Auth(cmd) => cmd.run(&opt).await,
//             Self::Run(cmd) => cmd.run(&opt).await,
//         }
//     }
// }

#[async_trait(?Send)]
pub trait ClientCommand {
    async fn run(&self, cfg: ClientConfig) -> anyhow::Result<()>;
}

#[async_trait(?Send)]
pub trait ServerCommand {
    async fn run(&self, cfg: ServerConfig) -> anyhow::Result<()>;
}

fn setup_logging(log_level: LogLevel) {
    use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};
    let level_filter: LevelFilter = log_level.into();

    TermLogger::init(
        level_filter,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

fn main() -> anyhow::Result<()> {
    let config = CliConfig::from_args();
    setup_logging(config.config.log_level.clone());
    actix_rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async {
        match config.command {
            Command::Client(cmd) => cmd.run(config.config.client).await,
            Command::Server(cmd) => cmd.run(config.config.server).await,
        }
    })
}
