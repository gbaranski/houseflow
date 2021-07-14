use houseflow_config::device::Config;

#[tokio::main]
async fn main() {
    houseflow_config::init_logging();
    let config = Config::get(Config::default_path())
        .await
        .expect("cannot load device config");
    houseflow_device::run(config, houseflow_device::devices::Light::default())
        .await
        .unwrap();
}
