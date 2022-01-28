use axum::AddExtensionLayer;
use axum_server::tls_rustls;
use houseflow_config::defaults;
use houseflow_config::server::Config;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;
use houseflow_server::clerk;
use houseflow_server::mailer;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    houseflow_config::log::init_with_config(houseflow_config::log::Config {
        hide_timestamp: std::env::var_os("HOUSEFLOW_LOG_HIDE_TIMESTAMP").is_some(),
    });
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
        "smtp" => mailer::smtp::Mailer::create(mailer::smtp::Config {
            host: config.email.url.host_str().unwrap().to_string(),
            port: config.email.url.port().unwrap_or(465),
            username: config.email.url.username().to_string(),
            password: urlencoding::decode(&config.email.url.password().unwrap().to_string())
                .unwrap()
                .to_string(),
            from: config.email.from.parse().unwrap(),
        }),
        scheme => panic!("unexpected email URL scheme: {}", scheme),
    };
    let clerk = clerk::sled::Clerk::new(defaults::clerk_path())?;
    let address = SocketAddr::new(config.network.address, config.network.port);
    let app = houseflow_server::app()
        .layer(AddExtensionLayer::new(clerk))
        .layer(AddExtensionLayer::new(mailer));

    if let Some(tls) = &config.tls {
        let fut = axum_server::bind(address).serve(app.clone().into_make_service());
        tracing::info!("Starting server at {}", address);

        let tls_address = SocketAddr::new(tls.address, tls.port);
        let tls_config =
            tls_rustls::RustlsConfig::from_pem_file(&tls.certificate, &tls.private_key).await?;
        let tls_fut = axum_server::bind_rustls(tls_address, tls_config)
            .serve(houseflow_server::app().into_make_service());
        tracing::info!("Starting TLS server at {}", tls_address);

        tokio::select! {
            val = fut => val?,
            val = tls_fut => val?
        };
    } else {
        let fut = axum_server::bind(address).serve(app.clone().into_make_service());
        tracing::info!("Starting server at {}", address);
        fut.await?;
    }

    Ok(())
}
