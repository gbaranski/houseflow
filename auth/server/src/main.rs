use actix_web::{web, App, HttpServer};

use token_store::RedisTokenStore;

use token::{exchange_refresh_token, exchange_refresh_token_query_config};
pub use token_store::TokenStore;

mod token;
mod token_store;

#[derive(Clone)]
pub struct AppState {
    token_store: RedisTokenStore,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const IP_ADDR: &str = "127.0.0.1:8080";
    env_logger::init();
    log::info!("Starting `Auth` service");
    let token_store = RedisTokenStore::new().await?;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/")
                    .app_data(token_store.clone())
                    .app_data(exchange_refresh_token_query_config())
                    .service(exchange_refresh_token),
            )
    })
    .bind(IP_ADDR)?;

    log::info!("Starting HTTP Server at `{}`", IP_ADDR);
    server.run().await?;
    Ok(())
}
