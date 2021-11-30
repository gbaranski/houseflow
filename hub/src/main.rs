use houseflow_config::hub::Config;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;
use houseflow_hub::providers::MasterProvider;
use houseflow_hub::providers::MijiaProvider;
use houseflow_hub::services::HapService;
use houseflow_hub::services::MasterService;
use houseflow_hub::Hub;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    houseflow_config::init_logging(false);
    let config_path = std::env::var("HOUSEFLOW_HUB_CONFIG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| Config::default_path());

    tracing::debug!("Config path: {}", config_path.to_str().unwrap());

    let config = match Config::read(&config_path) {
        Ok(config) => config,
        Err(ConfigError::IO(err)) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                tracing::error!(
                    "Config file could not be found at {}",
                    config_path.to_str().unwrap()
                );
                return Ok(());
            }
            _ => panic!("Read config IO Error: {}", err),
        },
        Err(err) => panic!("Config error: {}", err),
    };
    tracing::debug!("Config: {:#?}", config);
    let services = {
        let mut services = vec![];
        if let Some(hap_config) = config.providers.hap.as_ref() {
            services.push(Box::new(HapService::new(hap_config).await?) as _);
        }
        services
    };
    let (providers, provider_events) = {
        let mut providers = vec![];
        let (event_sender, event_receiver) = tokio::sync::mpsc::unbounded_channel();
        if let Some(mijia_config) = config.services.mijia {
            providers.push(Box::new(
                MijiaProvider::new(mijia_config, config.accessories.clone(), event_sender).await?,
            ) as _);
        }
        (providers, event_receiver)
    };
    let hub = Hub::new(
        MasterService::new(services),
        MasterProvider::new(providers),
        provider_events,
    )
    .await?;
    hub.run().await
}
