use actix_web::{web, HttpServer};
use houseflow_config::server::Config;
use houseflow_db::{sqlite::Database as SqliteDatabase, Database};
use houseflow_server::{Sessions, SledTokenStore, TokenStore};
use std::str::FromStr;
use std::sync::Arc;
use tracing::Level;

const LOG_ENV: &str = "HOUSEFLOW_LOG";

#[actix_web::main]
async fn main() {
    let level = std::env::var(LOG_ENV)
        .map(|env| {
            Level::from_str(env.to_uppercase().as_str())
                .expect(&format!("invalid `{}` environment variable", LOG_ENV))
        })
        .unwrap_or(Level::INFO);

    tracing_subscriber::fmt().with_max_level(level).init();
    let config = Config::get(Config::default_path())
        .await
        .expect("cannot load server config");
    let config = web::Data::new(config);
    let token_store = SledTokenStore::new(&config.tokens_path).expect("cannot open token store");
    let token_store = web::Data::from(Arc::new(token_store) as Arc<dyn TokenStore>);

    let database = SqliteDatabase::new(&config.database_path).expect("cannot open database");
    let database = web::Data::from(Arc::new(database) as Arc<dyn Database>);
    let address = config.address;
    let sessions = web::Data::new(Sessions::default());
    let server = HttpServer::new(move || {
        actix_web::App::new().configure(|cfg| {
            houseflow_server::configure(
                cfg,
                token_store.clone(),
                database.clone(),
                config.clone(),
                sessions.clone(),
            )
        })
    })
    .bind(address)
    .expect("bind address fail");
    server.run().await.expect("run server fail");
}
