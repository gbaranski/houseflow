use crate::{Command, ServerCommandState};
use async_trait::async_trait;

use self::auth::RunAuthCommand;
use self::fulfillment::RunFulfillmentCommand;
use self::lighthouse::RunLighthouseCommand;

mod auth;
mod fulfillment;
mod lighthouse;

use clap::Clap;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(Command)]
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
impl Command<ServerCommandState> for RunCommand {
    async fn run(&self, state: ServerCommandState) -> anyhow::Result<()> {
        match &self.service {
            Service::Auth(cmd) => cmd.run(state).await,
            Service::Lighthouse(cmd) => cmd.run(state).await,
            Service::Fulfillment(cmd) => cmd.run(state).await,
        }
    }
}
