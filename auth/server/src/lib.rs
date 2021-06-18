use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use db::Database;
use std::sync::Arc;
use types::ServerSecrets;

use crate::token::{exchange_refresh_token, exchange_refresh_token_form_config};
use ::token::store::TokenStore;
pub use config::Config;

mod auth;
pub mod config;
mod token;
mod whoami;

#[derive(Clone)]
pub struct AppState<TS: TokenStore, DB: Database> {
    token_store: TS,
    database: DB,
}

pub fn configure(
    cfg: &mut web::ServiceConfig,
    token_store: Data<dyn TokenStore>,
    database: Data<dyn Database>,
    config: Config,
    secrets: ServerSecrets,
) {
    cfg.data(secrets)
        .data(config)
        .app_data(token_store)
        .app_data(database)
        .service(auth::login::login)
        .service(auth::register::register)
        .service(auth::logout::logout)
        .service(whoami::whoami)
        .service(
            web::scope("/")
                .app_data(exchange_refresh_token_form_config)
                .service(exchange_refresh_token),
        );
}

pub async fn run(
    token_store: impl TokenStore + 'static,
    database: impl Database + 'static,
    config: Config,
    secrets: ServerSecrets,
) -> std::io::Result<()> {
    let token_store = Data::from(Arc::new(token_store) as Arc<dyn TokenStore>);
    let database = Data::from(Arc::new(database) as Arc<dyn Database>);

    let address = format!("{}:{}", config.host, config.port);
    log::info!("Starting Auth server at {}", address);

    let server = HttpServer::new(move || {
        App::new()
            .configure(|cfg| {
                crate::configure(
                    cfg,
                    token_store.clone(),
                    database.clone(),
                    config.clone(),
                    secrets.clone(),
                )
            })
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(address)?;

    server.run().await?;

    Ok(())
}

#[cfg(test)]
mod test_utils {
    use super::TokenStore;
    use token::store::MemoryTokenStore;

    use actix_web::web::Data;
    use rand::RngCore;
    use std::sync::Arc;

    pub const PASSWORD: &str = "SomePassword";
    pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub fn get_config() -> crate::Config {
        crate::Config::default()
    }

    pub fn get_secrets() -> types::ServerSecrets {
        let gen_secret = || {
            let mut bytes = [0; 32];
            rand::thread_rng().fill_bytes(&mut bytes);
            hex::encode(bytes)
        };
        types::ServerSecrets {
            refresh_key: gen_secret(),
            access_key: gen_secret(),
            password_salt: gen_secret(),
        }
    }

    pub fn get_token_store() -> Data<dyn TokenStore> {
        Data::from(Arc::new(MemoryTokenStore::new()) as Arc<dyn TokenStore>)
    }

    pub fn get_database() -> Data<dyn db::Database> {
        Data::from(Arc::new(db::memory::Database::new()) as Arc<dyn db::Database>)
    }
}
