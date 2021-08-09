use houseflow_config::{defaults, Error as ConfigError, server::Config, Config as _};
use houseflow_db::sqlite::Database as SqliteDatabase;
use houseflow_server::{Sessions, SledTokenStore};
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const HIDE_TIMESTAMP_ENV: &str = "HOUSEFLOW_SERVER_HIDE_TIMESTAMP";

    houseflow_config::init_logging(std::env::var_os(HIDE_TIMESTAMP_ENV).is_some());
    let config_path = std::env::var("HOUSEFLOW_SERVER_CONFIG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| Config::default_path());

    tracing::debug!("Config path: {}", config_path.to_str().unwrap());

    let config = match Config::read(&config_path) {
        Ok(config) => config,
        Err(ConfigError::IOError(err)) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                tracing::error!("Config file could not be found at {}", config_path.to_str().unwrap());
                return Ok(())
            },
            _ => panic!("Read config IO Error: {}", err),
        },
        Err(err) => panic!("Config error: {}", err),
    };
    tracing::debug!("Config: {:#?}", config);
    let token_store =
        SledTokenStore::new(defaults::token_store_path()).expect("cannot open token store");
    let database = SqliteDatabase::new(defaults::database_path()).expect("cannot open database");
    let sessions = Sessions::new();

    let state = houseflow_server::State {
        token_store: Arc::new(token_store),
        database: Arc::new(database),
        config: Arc::new(config),
        sessions: Arc::new(Mutex::new(sessions)),
    };

    let address = format!(
        "{}:{}",
        state.config.network.hostname,
        defaults::server_port()
    );
    let address = address
        .to_socket_addrs()
        .expect("invalid address")
        .next()
        .unwrap();
    tracing::debug!("{} address will be used", address);
    houseflow_server::run(&address, state).await;

    Ok(())
}
