use crate::{Command, Opt};
use async_trait::async_trait;
use structopt::StructOpt;

use auth::RunAuthCommand;
mod auth;

#[derive(StructOpt)]
pub enum Service {
    Auth(RunAuthCommand),
}

#[derive(StructOpt)]
pub struct RunCommand {
    #[structopt(subcommand)]
    pub service: Service,
}

#[async_trait(?Send)]
impl Command for RunCommand {
    async fn run(&self, opt: &Opt) -> anyhow::Result<()> {
        match self.service {
            Service::Auth(ref cmd) => cmd.run(opt).await,
        }
    }
}
