use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use db::Database;
use std::sync::Arc;
use types::UserAgent;

use token::store::TokenStore;

mod gactions;
mod internal;

#[derive(Clone)]
pub struct AppState<TS: TokenStore, DB: Database> {
    token_store: TS,
    database: DB,
}

#[derive(Clone)]
pub struct AppData {
    pub refresh_key: Vec<u8>,
    pub access_key: Vec<u8>,
}

#[derive(Clone)]
pub struct AgentData {
    user_agent: UserAgent,
}

pub(crate) fn config(
    cfg: &mut web::ServiceConfig,
    token_store: Data<dyn TokenStore>,
    database: Data<dyn Database>,
    app_data: AppData,
    internal_agent_data: AgentData,
) {
    cfg.data(app_data)
        .app_data(token_store)
        .app_data(database)
        .service(
            web::scope("/internal")
                .app_data(internal_agent_data.clone())
                .service(internal::on_sync),
        );
}

pub async fn run(
    address: impl std::net::ToSocketAddrs + std::fmt::Display + Clone,
    token_store: impl TokenStore + 'static,
    database: impl Database + 'static,
    app_data: AppData,
) -> std::io::Result<()> {
    let token_store = Data::from(Arc::new(token_store) as Arc<dyn TokenStore>);
    let database = Data::from(Arc::new(database) as Arc<dyn Database>);

    log::info!("Starting `Auth` service");

    let internal_agent_data = AgentData {
        user_agent: UserAgent::Internal,
    };

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(|cfg| {
                config(
                    cfg,
                    token_store.clone(),
                    database.clone(),
                    app_data.clone(),
                    internal_agent_data.clone(),
                )
            })
    })
    .bind(address.clone())?;

    log::info!("Starting HTTP Server at `{}`", address);

    server.run().await?;

    Ok(())
}

#[cfg(test)]
mod test_utils {
    use super::{Database, TokenStore};
    use db::MemoryDatabase;
    use token::store::MemoryTokenStore;

    use actix_web::web::Data;
    use rand::RngCore;
    use std::sync::Arc;

    pub const PASSWORD: &str = "SomePassword";
    pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub fn get_app_data() -> crate::AppData {
        let mut app_data = crate::AppData {
            refresh_key: vec![0; 32],
            access_key: vec![0; 32],
        };
        rand::thread_rng().fill_bytes(&mut app_data.refresh_key);
        rand::thread_rng().fill_bytes(&mut app_data.access_key);
        app_data
    }

    pub fn get_token_store() -> Data<dyn TokenStore> {
        Data::from(Arc::new(MemoryTokenStore::new()) as Arc<dyn TokenStore>)
    }

    pub fn get_database() -> Data<dyn Database> {
        Data::from(Arc::new(MemoryDatabase::new()) as Arc<dyn Database>)
    }
}
