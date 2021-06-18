use session::Session;
use types::{DeviceID, DevicePassword};
use url::Url;

mod session;

#[derive(Clone)]
pub struct Config {
    pub device_id: DeviceID,
    pub device_password: DevicePassword,
    pub lighthouse_url: Url,
}

pub async fn run(cfg: Config) -> anyhow::Result<()> {
    let session = Session::new(cfg);
    session.run().await?;

    Ok(())
}
