use crate::{Command, ServerCommandState};
use anyhow::Context;
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct RunFulfillmentCommand {}

#[async_trait(?Send)]
impl Command<ServerCommandState> for RunFulfillmentCommand {
    async fn run(&self, state: ServerCommandState) -> anyhow::Result<()> {
        let lighthouse = lighthouse::api::Lighthouse {
            host: state.config.lighthouse.host,
            port: state.config.lighthouse.port,
        };
        let database = db::postgres::Database::new(&state.config.postgres)
            .await
            .with_context(|| "connecting to postgres failed, is postgres on?")?;

        fulfillment::server::run(
            database,
            lighthouse,
            state.config.fulfillment,
            state.config.secrets,
        )
        .await?;

        Ok(())
    }
}
