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
    Register(auth::RegisterCommand),

    /// Run service/s
    Run(run::RunCommand),
}

#[derive(Clone)]
struct KeystorePath(PathBuf);

impl Default for KeystorePath {
    fn default() -> Self {
        Self(
            xdg::BaseDirectories::with_prefix("houseflow")
                .unwrap()
                .get_data_home()
                .join("token"),
        )
    }
}

impl std::ops::Deref for KeystorePath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<PathBuf> for KeystorePath {
    fn into(self) -> PathBuf {
        self.0
    }
}

impl From<&std::ffi::OsStr> for KeystorePath {
    fn from(str: &std::ffi::OsStr) -> Self {
        Self(PathBuf::from(str))
    }
}

impl std::fmt::Display for KeystorePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.display().fmt(f)
    }
}

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    command: Command,

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
    .block_on(async {
        match opt.command {
            Command::Login(ref command) => auth::login(&opt, command).await,
            Command::Register(ref command) => auth::register(&opt, command).await,
            Command::Run(ref command) => run::run(&opt, command).await,
        }
    })
}
