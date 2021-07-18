// use houseflow_config::device::Config;
// use houseflow_device::devices::light::{Device, State};
// use houseflow_types::DeviceStatus;
//
// fn on_off_hook(state: &mut State, on: bool) -> DeviceStatus {
//     tracing::info!("Changing `on` to {0}", on);
//     state.on = on;
//
//     DeviceStatus::Success
// }

#[tokio::main]
async fn main() {
    // houseflow_config::init_logging();
    // let config = Config::get(Config::default_path())
    //     .await
    //     .expect("cannot load device config");
    // let device = Device::new(on_off_hook);
    // houseflow_device::run(config, device).await.unwrap();
}
