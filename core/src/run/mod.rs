use crate::{ServerCommand, ServerConfig};
use async_trait::async_trait;

use self::auth::RunAuthCommand;
use self::fulfillment::RunFulfillmentCommand;
use self::lighthouse::RunLighthouseCommand;

mod auth;
mod fulfillment;
mod lighthouse;

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
