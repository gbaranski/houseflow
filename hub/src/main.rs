use houseflow_config::hub::Config;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;
use houseflow_hub::controllers::HapController;
use houseflow_hub::controllers::MasterController;
use houseflow_hub::providers::HiveProvider;
use houseflow_hub::providers::MasterProvider;
use houseflow_hub::providers::MijiaProvider;
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
    let (controllers, controller_events) = {
        let mut controllers = vec![];
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        if let Some(hap_config) = config.controllers.hap.as_ref() {
            controllers.push(Box::new(HapController::new(hap_config, tx).await?) as _);
        }
        if controllers.is_empty() {
            tracing::warn!("No controllers configured");
        }

        (controllers, rx)
    };
    let (providers, provider_events) = {
        let mut providers = vec![];
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        if let Some(mijia_config) = config.providers.mijia {
            providers.push(Box::new(
                MijiaProvider::new(mijia_config, config.accessories.clone(), tx.clone()).await?,
            ) as _);
        }
        if let Some(hive_config) = config.providers.hive {
            providers.push(Box::new(
                HiveProvider::new(hive_config, config.accessories.clone(), tx.clone()).await?,
            ) as _)
        }
        if providers.is_empty() {
            tracing::warn!("No providers configured");
        }
        (providers, rx)
    };
    let hub = Hub::new(
        MasterController::new(controllers),
        MasterProvider::new(providers),
    )
    .await?;
    hub.run(provider_events, controller_events).await
}
