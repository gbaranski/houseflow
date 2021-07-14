use crate::{Command, ServerCommandState};
use actix_web::{web::Data, App, HttpServer};
use async_trait::async_trait;
use houseflow_db::{sqlite::Database as SqliteDatabase, Database};
use houseflow_server::{SledTokenStore, TokenStore};
use std::sync::Arc;

use clap::Clap;

#[derive(Clap)]
pub struct RunServerCommand {}

#[async_trait(?Send)]
impl Command<ServerCommandState> for RunServerCommand {
    async fn run(self, state: ServerCommandState) -> anyhow::Result<()> {
        let token_store = SledTokenStore::new(&state.config.tokens_path)?;
        let token_store = Data::from(Arc::new(token_store) as Arc<dyn TokenStore>);

        let database = SqliteDatabase::new(&state.config.database_path)?;
        let database = Data::from(Arc::new(database) as Arc<dyn Database>);

        let address = (
            state.config.hostname.to_string(),
            houseflow_config::defaults::server_port(),
        );
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
