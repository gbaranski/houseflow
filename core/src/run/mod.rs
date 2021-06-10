use crate::{ServerCommand, ServerConfig};
use async_trait::async_trait;

use auth::RunAuthCommand;
use lighthouse::RunLighthouseCommand;
mod auth;
mod lighthouse;

use clap::Clap;

#[derive(Clap)]
pub enum Service {
    Auth(RunAuthCommand),
    Lighthouse(RunLighthouseCommand),
}

#[derive(Clap)]
pub struct RunCommand {
    #[clap(subcommand)]
    pub service: Service,
}

#[async_trait(?Send)]
impl ServerCommand for RunCommand {
    async fn run(&self, cfg: ServerConfig) -> anyhow::Result<()> {
        match &self.service {
            Service::Auth(cmd) => cmd.run(cfg).await,
            Service::Lighthouse(cmd) => cmd.run(cfg).await,
        }
    }
}
