use houseflow_accessory_virtual::VirtualAccessory;
use houseflow_api::hub::hive::Session;
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
    let hive_session = Session::new(config.hub.url, config.credentials);
    let (event_sender, event_receiver) = tokio::sync::mpsc::unbounded_channel();
    let accessory = VirtualAccessory::new(event_sender);
    hive_session.run(&accessory, event_receiver).await?;

    Ok(())
}
