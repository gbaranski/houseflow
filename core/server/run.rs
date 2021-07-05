use crate::{Command, ServerCommandState};
use actix_web::{web::Data, App, HttpServer};
use anyhow::Context;
use async_trait::async_trait;
use houseflow_db::{sqlite::Database as SqliteDatabase, Database};
use houseflow_server::{RedisTokenStore, TokenStore};
use std::sync::Arc;

use clap::Clap;

#[derive(Clap)]
pub struct RunServerCommand {}

#[async_trait(?Send)]
impl Command<ServerCommandState> for RunServerCommand {
    async fn run(self, state: ServerCommandState) -> anyhow::Result<()> {
        let token_store = RedisTokenStore::new()
            .await
            .with_context(|| "connect to redis failed, is redis on?")?;
        let token_store = Data::from(Arc::new(token_store) as Arc<dyn TokenStore>);

        let database = SqliteDatabase::new_in_memory()?;
        let database = Data::from(Arc::new(database) as Arc<dyn Database>);

        let address = state.config.address;
        let config = Data::new(state.config);
        let sessions = Data::new(houseflow_server::Sessions::default());
        let server = HttpServer::new(move || {
            App::new()
                .wrap(actix_web::middleware::Logger::default())
                .configure(|cfg| {
                    houseflow_server::configure(
                        cfg,
                        token_store.clone(),
                        database.clone(),
                        config.clone(),
                        sessions.clone(),
                    )
                })
        })
        .bind(address)?;
        server.run().await?;
        Ok(())
    }
}
