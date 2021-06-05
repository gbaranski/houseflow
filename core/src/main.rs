use structopt::StructOpt;
use url::Url;
use std::path::PathBuf;

mod auth;
mod cli;

#[derive(StructOpt)]
enum Command {
    /// Log in to existing Houseflow account
    Login,

    /// Register new Houseflow account
    Register,
}

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    command: Command,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Path to Token store, used to store persistant sessions
    #[structopt(long, default_value = "~/.cache/houseflow/token")]
    token_store_path: PathBuf,

    /// URL of the Auth service
    #[structopt(long, default_value = "http://localhost:8080")]
    auth_url: Url,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose as u64)?;

    match opt.command {
        Command::Login => auth::login(opt),
        Command::Register => todo!(),
    }
    .await
}
