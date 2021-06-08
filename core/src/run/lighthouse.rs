use crate::{ServerConfig, ServerCommand};
use async_trait::async_trait;
use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct RunLighthouseCommand {
    #[structopt(long, default_value = "127.0.0.1:6002")]
    pub address: SocketAddr,
}

#[async_trait(?Send)]
impl ServerCommand for RunLighthouseCommand {
    async fn run(&self, _cfg: ServerConfig) -> anyhow::Result<()> {
        lighthouse_broker::run(self.address).await?;

        Ok(())
    }
}
