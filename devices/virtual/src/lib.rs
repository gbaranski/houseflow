use houseflow_config::device::Config;
use session::Session;

pub mod devices;
mod session;

pub async fn run<D: devices::Device<EP>, EP: devices::ExecuteParams>(
    cfg: Config,
    device: D,
) -> anyhow::Result<()> {
    let session = Session::new(cfg);
    session.run(device).await?;

    Ok(())
}
