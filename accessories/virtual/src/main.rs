use houseflow_accessory_virtual::VirtualAccessory;
use houseflow_api::hub::hive::HiveClient;
use houseflow_config::accessory::Config;
use houseflow_config::Config as _;
use houseflow_config::Error as ConfigError;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    houseflow_config::log::init();
    let config_path = std::env::var("HOUSEFLOW_ACCESSORY_CONFIG")
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
    let (_, future) = HiveClient::connect(
        |client| VirtualAccessory::new(client, config.services),
        config.credentials,
        config.hub.url,
    )
    .await;
    future.await.unwrap();

    Ok(())
}
