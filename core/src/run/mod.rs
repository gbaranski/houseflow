use crate::{ServerCommand, ServerConfig};
use async_trait::async_trait;

use auth::RunAuthCommand;
use self::lighthouse::RunLighthouseCommand;
use self::fulfillment::RunFulfillmentCommand;

mod auth;
mod lighthouse;
mod fulfillment;

use clap::Clap;

#[derive(Clap)]
pub enum Service {
    Auth(RunAuthCommand),
    Lighthouse(RunLighthouseCommand),
    Fulfillment(RunFulfillmentCommand),
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
            Service::Fulfillment(cmd) => cmd.run(cfg).await,
        }
    }
}
