use crate::{ServerCommand, ServerConfig};
use async_trait::async_trait;
use structopt::StructOpt;

use auth::RunAuthCommand;
use lighthouse::RunLighthouseCommand;
mod auth;
mod lighthouse;

#[derive(StructOpt)]
pub enum Service {
    Auth(RunAuthCommand),
    Lighthouse(RunLighthouseCommand),
}

#[derive(StructOpt)]
pub struct RunCommand {
    #[structopt(subcommand)]
    pub service: Service,
}

#[async_trait(?Send)]
impl ServerCommand for RunCommand {
    async fn run(&self, cfg: ServerConfig) -> anyhow::Result<()> {
        match self.service {
            Service::Auth(ref cmd) => cmd.run(cfg).await,
            Service::Lighthouse(ref cmd) => cmd.run(cfg).await,
        }
    }
}
