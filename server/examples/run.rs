use actix_web::{web, HttpServer};
use houseflow_config::server::Config;
use houseflow_db::{sqlite::Database as SqliteDatabase, Database};
use houseflow_server::{Sessions, SledTokenStore, TokenStore};
use std::sync::Arc;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::default().filter_or("HOUSEFLOW_LOG", "info"));

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
        actix_web::App::new()
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
    .bind(address)
    .expect("bind address fail");
    server.run().await.expect("run server fail");
}
