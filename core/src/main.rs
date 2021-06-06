use async_trait::async_trait;
use auth::{AuthCommand, KeystorePath};
use run::RunCommand;
use structopt::StructOpt;
use url::Url;

mod auth;
mod cli;
mod run;

#[derive(StructOpt)]
enum RootCommand {
    /// Login, register, logout, and refresh your authentication
    Auth(AuthCommand),

    /// Run service/s
    Run(RunCommand),
}

#[async_trait(?Send)]
impl Command for RootCommand {
    async fn run(&self, opt: &Opt) -> anyhow::Result<()> {
        match self {
            Self::Auth(cmd) => cmd.run(&opt).await,
            Self::Run(cmd) => cmd.run(&opt).await,
        }
    }
}

#[async_trait(?Send)]
pub trait Command {
    async fn run(&self, opt: &Opt) -> anyhow::Result<()>;
}

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    command: RootCommand,

    /// Path to Keystore, used to store persistant sessions
    #[structopt(long, default_value, parse(from_os_str))]
    keystore_path: KeystorePath,

    /// URL of the Auth service
    #[structopt(long, default_value = "http://localhost:6001")]
    auth_url: Url,

    /// Enable debug logging
    #[structopt(long)]
    debug: bool,

    /// Enable trace logging
    #[structopt(long)]
    trace: bool,
}

fn setup_logging(opt: &Opt) {
    use simplelog::{ColorChoice, LevelFilter, TermLogger, TerminalMode};

    let level_filter = if opt.trace {
        LevelFilter::Trace
    } else if opt.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    TermLogger::init(
        level_filter,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    setup_logging(&opt);
    actix_rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async { opt.command.run(&opt).await })
}
