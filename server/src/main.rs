use axum_server::tls_rustls;
use houseflow_config::defaults;
use houseflow_config::server::Config;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;
use houseflow_server::clerk::sled::Clerk;
use houseflow_server::mailer;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const HIDE_TIMESTAMP_ENV: &str = "HOUSEFLOW_SERVER_HIDE_TIMESTAMP";

    houseflow_config::init_logging(std::env::var_os(HIDE_TIMESTAMP_ENV).is_some());
    let config_path = std::env::var("HOUSEFLOW_SERVER_CONFIG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| Config::default_path());

    tracing::debug!("Config path: {:?}", config_path);

    let config = match Config::read(&config_path) {
        Ok(config) => config,
        Err(ConfigError::IO(err)) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                tracing::error!("Config file could not be found at {:?}", config_path);
                return Ok(());
            }
            _ => panic!("Read config IO Error: {}", err),
        },
        Err(err) => panic!("Config error: {}", err),
    };
    tracing::debug!("Config: {:#?}", config);
    let mailer = match config.email.url.scheme() {
        "smtp" => mailer::smtp::Mailer::new(mailer::smtp::Config {
            host: config.email.url.host_str().unwrap().to_string(),
            port: config.email.url.port().unwrap_or(465),
            username: config.email.url.username().to_string(),
            password: urlencoding::decode(&config.email.url.password().unwrap().to_string())
                .unwrap()
                .to_string(),
            from: config.email.from.clone(),
        }),
        scheme => panic!("unexpected email URL scheme: {}", scheme),
    };
    let clerk = Clerk::new(defaults::clerk_path())?;
    let state = houseflow_server::State {
        config: Arc::new(config),
        sessions: Default::default(),
        mailer: Arc::new(mailer),
        clerk: Arc::new(clerk),
    };

    let address_with_port = |address, port| std::net::SocketAddr::new(address, port);
    let address = address_with_port(state.config.network.address, state.config.network.port);

    if let Some(tls) = &state.config.tls {
        let fut = axum_server::bind(address)
            .serve(houseflow_server::app(state.clone()).into_make_service());
        tracing::info!("Starting server at {}", address);

        let tls_address = address_with_port(tls.address, tls.port);
        let tls_config =
            tls_rustls::RustlsConfig::from_pem_file(&tls.certificate, &tls.private_key).await?;
        let tls_fut = axum_server::bind_rustls(tls_address, tls_config)
            .serve(houseflow_server::app(state).into_make_service());
        tracing::info!("Starting TLS server at {}", tls_address);

        tokio::select! {
            val = fut => val?,
            val = tls_fut => val?
        };
    } else {
        let fut =
            axum_server::bind(address).serve(houseflow_server::app(state).into_make_service());
        tracing::info!("Starting server at {}", address);
        fut.await?;
    }

    Ok(())
}
