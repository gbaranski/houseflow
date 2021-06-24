use actix_web::web::{self, Data};
use config::server::Secrets;
use db::Database;

use crate::token::{exchange_refresh_token, exchange_refresh_token_form_config};
use ::token::store::TokenStore;

mod auth;
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
    secrets: Secrets,
) {
    cfg.data(secrets)
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

#[cfg(test)]
mod test_utils {
    use super::TokenStore;
    use token::store::MemoryTokenStore;

    use actix_web::web::Data;
    use std::sync::Arc;

    pub const PASSWORD: &str = "SomePassword";
    pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub fn get_token_store() -> Data<dyn TokenStore> {
        Data::from(Arc::new(MemoryTokenStore::new()) as Arc<dyn TokenStore>)
    }

    pub fn get_database() -> Data<dyn db::Database> {
        Data::from(Arc::new(db::memory::Database::new()) as Arc<dyn db::Database>)
    }
}
