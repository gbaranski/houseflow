use houseflow_config::hub::Config;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;
use houseflow_hub::controllers;
use houseflow_hub::providers;
use houseflow_hub::providers::ProviderName;
use houseflow_hub::Hub;
use tokio::sync::mpsc;

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

    let (provider_tx, provider_rx) = mpsc::channel(8);
    let mut master_provider = providers::Master::new(provider_rx);
    let provider = providers::ProviderHandle::new(ProviderName::Master, provider_tx);

    let (controller_tx, controller_rx) = mpsc::channel(8);
    let mut master_controller = controllers::Master::new(controller_rx);
    let controller = controllers::ControllerHandle::new("master", controller_tx);

    {
        // TODO: Simplify that
        if let Some(mijia_config) = config.providers.mijia {
            let handle =
                providers::Mijia::new(controller.clone(), mijia_config, config.accessories.clone())
                    .await?;
            master_provider.insert(handle);
        }
        if let Some(hive_config) = config.providers.hive {
            let handle =
                providers::Hive::new(controller.clone(), hive_config, config.accessories.clone());
            master_provider.insert(handle.into());
        }
    };

    {
        if let Some(hap_config) = config.controllers.hap {
            let handle = controllers::Hap::new(provider.clone(), hap_config).await?;
            master_controller.insert(handle);
        }
    };

    tokio::spawn(async move {
        master_provider.run().await.unwrap();
    });

    tokio::spawn(async move {
        master_controller.run().await.unwrap();
    });

    let hub = Hub::new(controller, provider).await?;
    hub.run().await
}
