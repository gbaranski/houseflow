use crate::{Command, ServerCommandState};
use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Context;
use async_trait::async_trait;
use db::Database;
use std::sync::Arc;
use token::store::TokenStore;
use url::Url;

use clap::Clap;

#[derive(Clap)]
pub struct RunServerCommand {}

#[async_trait(?Send)]
impl Command<ServerCommandState> for RunServerCommand {
    async fn run(&self, state: ServerCommandState) -> anyhow::Result<()> {
        let address = format!("{}:{}", state.config.host, state.config.port);
        let base_url = Url::parse(&format!("http://{}", address)).unwrap();
        let token_store = || async {
            token::store::RedisTokenStore::new()
                .await
                .with_context(|| "connect to redis failed, is redis on?")
        };

        let database = || async {
            db::postgres::Database::new(&state.config.postgres)
                .await
                .with_context(|| "connect to postgres failed, is redis on?")
        };

        let lighthouse_api =
            || lighthouse::api::Lighthouse::new(base_url.join("lighthouse/").unwrap());

        let database = Data::from(Arc::from(database().await?) as Arc<dyn Database>);
        let token_store = Data::from(Arc::from(token_store().await?) as Arc<dyn TokenStore>);
        let lighthouse_api = Data::from(
            Arc::from(lighthouse_api()) as Arc<dyn lighthouse::api::prelude::Lighthouse>
        );
        let lighthouse_app_data = Data::from(Arc::from(lighthouse::server::AppState::default()));

        let server = HttpServer::new(move || {
            App::new()
                .wrap(actix_web::middleware::Logger::default())
                .service(web::scope("/auth").configure(|cfg| {
                    auth::server::configure(
                        cfg,
                        token_store.clone(),
                        database.clone(),
                        state.config.auth.clone(),
                        state.config.secrets.clone(),
                    )
                }))
                .service(web::scope("/lighthouse").configure(|cfg| {
                    lighthouse::server::configure(
                        cfg,
                        lighthouse_app_data.clone(),
                        database.clone(),
                    )
                }))
                .service(web::scope("/fulfillment").configure(|cfg| {
                    fulfillment::server::configure(
                        cfg,
                        database.clone(),
                        lighthouse_api.clone(),
                        state.config.fulfillment.clone(),
                        state.config.secrets.clone(),
                    )
                }))
        })
        .bind(address)?;
        server.run().await?;
        Ok(())
    }
}
