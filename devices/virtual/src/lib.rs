use session::Session;
use types::{DeviceID, DevicePassword};
use url::Url;

pub mod devices;
mod session;

#[derive(Clone)]
pub struct Config {
    pub device_id: DeviceID,
    pub device_password: DevicePassword,
    pub lighthouse_url: Url,
}

pub async fn run<D: devices::Device<EP>, EP: devices::ExecuteParams>(
    cfg: Config,
    device: D,
) -> anyhow::Result<()> {
    let session = Session::new(cfg);
    session.run(device).await?;

    Ok(())
}
