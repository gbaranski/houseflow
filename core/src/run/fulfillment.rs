use crate::{ServerCommand, ServerConfig};
use anyhow::Context;
use async_trait::async_trait;

use clap::Clap;

#[derive(Clap)]
pub struct RunFulfillmentCommand {}

#[async_trait(?Send)]
impl ServerCommand for RunFulfillmentCommand {
    async fn run(&self, config: ServerConfig) -> anyhow::Result<()> {
        use std::net::{Ipv4Addr, SocketAddrV4};

        let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), config.fulfillment.port);
        let database_config = db::PostgresConfig {
            user: "postgres",
            password: "",
            host: "localhost",
            port: 5432,
            database_name: "houseflow",
        };
        let lighthouse = lighthouse_api::Lighthouse {
            url: url::Url::parse("http://127.0.0.1:6002").unwrap(),
        };
        let database = db::PostgresDatabase::new(&database_config)
            .await
            .with_context(|| "connecting to postgres failed, is postgres on?")?;

        let app_data = fulfillment_server::AppData {
            refresh_key: config.refresh_key.into(),
            access_key: config.access_key.into(),
        };
        fulfillment_server::run(address, database, lighthouse, app_data).await?;

        Ok(())
    }
}
