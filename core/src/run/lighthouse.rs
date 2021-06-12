use crate::{ServerCommand, ServerConfig};
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct RunLighthouseCommand {}

#[async_trait(?Send)]
impl ServerCommand for RunLighthouseCommand {
    async fn run(&self, cfg: ServerConfig) -> anyhow::Result<()> {
        let address = std::net::SocketAddr::new(
            std::net::Ipv4Addr::new(127, 0, 0, 1).into(),
            cfg.lighthouse.port,
        );
        lighthouse_server::run(address).await?;

        Ok(())
    }
}
