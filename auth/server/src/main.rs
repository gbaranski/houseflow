use actix_web::{App, HttpServer};

use token_store::RedisTokenStore;

use token::exchange_refresh_token;

mod token;
mod token_store;

#[derive(Clone)]
pub struct AppState {
    token_store: RedisTokenStore,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const IP_ADDR: &str = "127.0.0.1:8080";
    env_logger::init();
    log::info!("Starting `Auth` service");

    let server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(exchange_refresh_token)
    })
    .bind(IP_ADDR)?;

    log::info!("Starting HTTP Server at `{}`", IP_ADDR);
    server.run().await?;
    Ok(())
}
