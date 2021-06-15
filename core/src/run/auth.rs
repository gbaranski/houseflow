use crate::{ServerCommand, ServerConfig};
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
impl ServerCommand for RunAuthCommand {
    async fn run(&self, config: ServerConfig) -> anyhow::Result<()> {
        use std::net::{Ipv4Addr, SocketAddrV4};

        let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), config.auth.port);
        let token_store = RedisTokenStore::new()
            .await
            .with_context(|| "connect to redis failed, is redis on?")?;
        let database_config = db::PostgresConfig {
            user: "postgres",
            password: "",
            host: "localhost",
            port: 5432,
            database_name: "houseflow",
        };
        let database = db::PostgresDatabase::new(&database_config)
            .await
            .with_context(|| "connecting to postgres failed, is postgres on?")?;
        let app_data = auth::server::AppData {
            refresh_key: config.refresh_key.into(),
            access_key: config.access_key.into(),
            password_salt: config.auth.password_salt.into(),
        };
        auth::server::run(address, token_store, database, app_data).await?;

        Ok(())
    }
}
