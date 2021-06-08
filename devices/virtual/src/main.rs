use session::{Options as SessionOptions, Session};
use structopt::StructOpt;
use types::{DeviceID, DevicePassword};
use url::Url;

mod session;

/// Houseflow device
#[derive(StructOpt, Debug)]
#[structopt(name = "houseflow-device")]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// URL of websocket endpoint of Lighthouse
    #[structopt(name = "LIGHTHOUSE_URL")]
    lighthouse_url: Url,

    /// ID of the device, must match with ID in database
    #[structopt(long)]
    device_id: DeviceID,

    /// Password of the device, must match with Password in database
    #[structopt(long)]
    device_password: DevicePassword,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose.into()).expect("failed initializing verbosity");
    log::info!("Verbosity: {}", opt.verbose);
    println!("Lighthouse URL: {:?}", opt.lighthouse_url);
    let session = Session::new(SessionOptions {
        url: opt.lighthouse_url,
        id: opt.device_id,
        password: opt.device_password,
    });
    session.run().await?;

    Ok(())
}
