use crate::{Command, ServerCommandState};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct RunLighthouseCommand {}

#[async_trait(?Send)]
impl Command<ServerCommandState> for RunLighthouseCommand {
    async fn run(&self, state: ServerCommandState) -> anyhow::Result<()> {
        let address = std::net::SocketAddr::new(
            std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
            state.config.lighthouse.port,
        );
        lighthouse::server::run(address).await?;

        Ok(())
    }
}
