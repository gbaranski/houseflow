use crate::{Command, ServerCommandState};
use anyhow::Context;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use token::store::RedisTokenStore;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthServerConfig {
    password_salt: String,
}

use clap::Clap;

#[derive(Clap)]
pub struct RunAuthCommand {}

#[async_trait(?Send)]
impl Command<ServerCommandState> for RunAuthCommand {
    async fn run(&self, state: ServerCommandState) -> anyhow::Result<()> {
        let token_store = RedisTokenStore::new()
            .await
            .with_context(|| "connect to redis failed, is redis on?")?;
        let database = db::postgres::Database::new(&state.config.postgres)
            .await
            .with_context(|| "connecting to postgres failed, is postgres on?")?;
        auth::server::run(
            token_store,
            database,
            state.config.auth,
            state.config.secrets,
        )
        .await?;

        Ok(())
    }
}
