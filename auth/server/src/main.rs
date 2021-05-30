// TODO: remove that
#![allow(unused_imports)]

use actix_web::{web, App, HttpServer};

pub use token_store::{
    MemoryTokenStore, MemoryTokenStoreError, RedisTokenStore, RedisTokenStoreError,
};

use token::{exchange_refresh_token, exchange_refresh_token_form_config};
pub use token_store::TokenStore;

mod token;
mod token_store;

#[derive(Clone)]
pub struct AppState {
    token_store: RedisTokenStore,
}

pub fn config(cfg: &mut web::ServiceConfig, token_store: web::Data<Box<dyn TokenStore>>) {
    cfg.app_data(token_store).service(exchange_refresh_token);
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const IP_ADDR: &str = "127.0.0.1:8080";
    env_logger::init();
    log::info!("Starting `Auth` service");
    let token_store: web::Data<Box<dyn TokenStore>> =
        web::Data::new(Box::new(MemoryTokenStore::new()));

    let server = HttpServer::new(move || {
        App::new()
            .configure(|cfg| config(cfg, token_store.clone()))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(IP_ADDR)?;

    log::info!("Starting HTTP Server at `{}`", IP_ADDR);
    server.run().await?;
    Ok(())
}
