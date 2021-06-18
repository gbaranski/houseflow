use session::Session;
use types::{DeviceID, DevicePassword};

mod session;

#[derive(Clone)]
pub struct LighthouseConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone)]
pub struct Config {
    pub device_id: DeviceID,
    pub device_password: DevicePassword,
    pub lighthouse: LighthouseConfig,
}

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let session = Session::new(cfg);
    session.run().await?;

    Ok(())
}
