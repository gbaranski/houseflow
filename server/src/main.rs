use axum::AddExtensionLayer;
use axum_server::tls_rustls;
use houseflow_config::defaults;
use houseflow_config::server::Config;
use houseflow_config::server::Controllers;
use houseflow_config::server::Providers;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;
use houseflow_server::clerk;
use houseflow_server::controllers;
use houseflow_server::mailer;
use houseflow_server::providers;
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

    let (provider_tx, provider_rx) = acu::channel(8, providers::Name::Master.into());
    let mut master_provider = providers::Master::new(provider_rx);
    let provider = providers::Handle::new(providers::Name::Master, provider_tx);

    let (controller_tx, controller_rx) = acu::channel(8, controllers::Name::Master.into());
    let mut master_controller = controllers::Master::new(controller_rx);
    let controller = controllers::Handle::new(controllers::Name::Master, controller_tx);

    let mut app = houseflow_server::app()
        .layer(AddExtensionLayer::new(clerk))
        .layer(AddExtensionLayer::new(mailer))
        .layer(AddExtensionLayer::new(provider.clone()))
        .layer(AddExtensionLayer::new(controller.clone()));

    let Providers { lighthouse } = config.providers;
    if let Some(config) = lighthouse {
        let handle = providers::lighthouse::LighthouseProvider::create(controller.clone(), config);
        app = app.layer(AddExtensionLayer::new(handle.clone()));
        master_provider.insert(handle.into());
    }

    let Controllers { meta } = config.controllers;
    if let Some(config) = meta {
        let handle = controllers::meta::MetaController::create(provider.clone(), config);
        app = app.layer(AddExtensionLayer::new(handle.clone()));
        master_controller.insert(handle.into());
    }

    tokio::spawn(async move {
        master_provider.run().await.unwrap();
    });

    tokio::spawn(async move {
        master_controller.run().await.unwrap();
    });

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
