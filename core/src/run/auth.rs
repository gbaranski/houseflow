use crate::{ServerCommand, ServerConfig};
use async_trait::async_trait;
use token::store::MemoryTokenStore;
use serde::{Serialize, Deserialize};

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
        use std::net::{SocketAddrV4, Ipv4Addr};

        let address = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), config.auth.port);
        let token_store = MemoryTokenStore::new();
        let database = db::MemoryDatabase::new();
        let app_data = auth_server::AppData {
            refresh_key: config.refresh_key.into(),
            access_key: config.access_key.into(),
            password_salt: config.auth.password_salt.into(),
        };
        auth_server::run(address, token_store, database, app_data).await?;

        Ok(())
    }
}
