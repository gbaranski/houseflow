use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use houseflow_db::{Database, MemoryDatabase};
use std::sync::Arc;

pub use token_store::{
    MemoryTokenStore, MemoryTokenStoreError, RedisTokenStore, RedisTokenStoreError,
};

use token::{exchange_refresh_token, exchange_refresh_token_form_config};
pub use token_store::TokenStore;

mod auth;
mod token;
mod token_store;

#[derive(Clone)]
pub struct AppState<TS: TokenStore, DB: Database> {
    token_store: TS,
    database: DB,
}

#[derive(Clone)]
pub struct AppData {
    refresh_key: Vec<u8>,
    access_key: Vec<u8>,
    password_salt: Vec<u8>,
}

pub fn config(
    cfg: &mut web::ServiceConfig,
    token_store: Data<dyn TokenStore>,
    database: Data<dyn Database>,
    app_data: AppData,
) {
    cfg.data(app_data)
        .app_data(token_store)
        .app_data(database)
        .service(auth::login)
        .service(auth::register)
        .service(
            web::scope("/")
                .app_data(exchange_refresh_token_form_config)
                .service(exchange_refresh_token),
        );
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const IP_ADDR: &str = "127.0.0.1:8080";
    env_logger::init();
    log::info!("Starting `Auth` service");

    let token_store = Data::from(Arc::new(MemoryTokenStore::new()) as Arc<dyn TokenStore>);
    let database = Data::from(Arc::new(MemoryDatabase::new()) as Arc<dyn Database>);

    let app_data = AppData {
        refresh_key: Vec::from("refresh-key"),
        access_key: Vec::from("access-key"),
        password_salt: Vec::from("sea-salt"),
    };

    let server = HttpServer::new(move || {
        App::new()
            .configure(|cfg| config(cfg, token_store.clone(), database.clone(), app_data.clone()))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(IP_ADDR)?;

    log::info!("Starting HTTP Server at `{}`", IP_ADDR);
    server.run().await?;
    Ok(())
}

#[cfg(test)]
mod test_utils {
    use super::{Database, MemoryDatabase, MemoryTokenStore, TokenStore};
    
    use actix_web::web::Data;
    use rand::RngCore;
    use std::sync::Arc;

    pub fn get_app_data() -> crate::AppData {
        let mut app_data = crate::AppData {
            refresh_key: vec![0; 32],
            access_key: vec![0; 32],
            password_salt: vec![0; 32],
        };
        rand::thread_rng().fill_bytes(&mut app_data.refresh_key);
        rand::thread_rng().fill_bytes(&mut app_data.access_key);
        rand::thread_rng().fill_bytes(&mut app_data.password_salt);
        app_data
    }

    pub fn get_token_store() -> Data<dyn TokenStore> {
        Data::from(Arc::new(MemoryTokenStore::new()) as Arc<dyn TokenStore>)
    }

    pub fn get_database() -> Data<dyn Database> {
        Data::from(Arc::new(MemoryDatabase::new()) as Arc<dyn Database>)
    }
}
