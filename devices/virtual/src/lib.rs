use session::{Options as SessionOptions, Session};
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
    let session = Session::new(SessionOptions {
        url: cfg.lighthouse_url,
        id: cfg.device_id,
        password: cfg.device_password,
    });
    session.run().await?;

    Ok(())
}
