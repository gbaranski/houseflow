use std::path::PathBuf;
use structopt::StructOpt;
use url::Url;

mod auth;
mod cli;
mod run;

#[derive(StructOpt)]
enum Command {
    /// Log in to existing Houseflow account
    Login(auth::LoginCommand),

    /// Register new Houseflow account
    Register,

    /// Run service/s
    Run(run::RunCommand),
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
    #[structopt(long, default_value = "http://localhost:6001")]
    auth_url: Url,
}

fn main() -> anyhow::Result<()> {
    actix_rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async {
        let opt = Opt::from_args();
        loggerv::init_with_verbosity(opt.verbose as u64)?;

        match opt.command {
            Command::Login(ref command) => auth::login(&opt, command).await,
            Command::Register => todo!(),
            Command::Run(ref command) => run::run(&opt, command).await,
        }
    })
}
