use houseflow_config::device::Config;
use std::str::FromStr;
use tracing::Level;

const LOG_ENV: &str = "HOUSEFLOW_LOG";

#[tokio::main]
async fn main() {
    let level = std::env::var(LOG_ENV)
        .map(|env| {
            Level::from_str(env.to_uppercase().as_str())
                .expect(&format!("invalid `{}` environment variable", LOG_ENV))
        })
        .unwrap_or(Level::INFO);

    tracing_subscriber::fmt().with_max_level(level).init();
    let config = Config::get(Config::default_path())
        .await
        .expect("cannot load server config");
    houseflow_device::run(config, houseflow_device::devices::Light::default()).await.unwrap();
}
