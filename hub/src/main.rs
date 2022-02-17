use acu::MasterExt;
use houseflow_config::hub::Config;
use houseflow_config::hub::Controllers;
use houseflow_config::hub::Providers;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;
use houseflow_hub::controllers;
use houseflow_hub::providers;
use houseflow_hub::Hub;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    houseflow_config::log::init();
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

    let master_provider = providers::MasterHandle::new();

    let master_controller = controllers::MasterHandle::new();
    let controller: controllers::Handle = master_controller.clone().into();

    {
        let Providers { hive, mijia } = config.providers;
        // TODO: Simplify that
        if let Some(mijia_config) = mijia {
            let controller: controllers::Handle = master_controller.clone().into();
            let handle =
                providers::mijia::new(controller, mijia_config, config.accessories.clone()).await?;
            master_provider.push(handle).await;
        }
        if let Some(hive_config) = hive {
            let handle =
                providers::hive::new(controller.clone(), hive_config, config.accessories.clone());
            master_provider.push(handle.into()).await;
        }
    };

    let Controllers { hap, lighthouse } = config.controllers;
    // Insert configured controllers
    if let Some(hap_config) = hap {
        let handle = controllers::hap::new(master_provider.clone(), hap_config).await?;
        master_controller.push(handle).await;
    }

    if let Some(lighthouse_config) = lighthouse {
        let handle =
            controllers::lighthouse::new(master_provider.clone(), config.hub.id, lighthouse_config)
                .await?;
        master_controller.push(handle).await;
    }

    let hub = Hub::new(master_controller, master_provider).await?;
    hub.run().await
}
