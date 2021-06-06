use crate::{Command, Opt};
use async_trait::async_trait;
use structopt::StructOpt;

use login::LoginCommand;
use register::RegisterCommand;
use status::StatusCommand;

mod login;
mod register;
mod status;

#[derive(StructOpt)]
pub enum AuthCommand {
    /// Log in to existing Houseflow account
    Login(LoginCommand),

    /// Register a new Houseflow account
    Register(RegisterCommand),

    Status(StatusCommand),
}

#[async_trait(?Send)]
impl Command for AuthCommand {
    async fn run(&self, opt: &Opt) -> anyhow::Result<()> {
        match self {
            Self::Login(cmd) => cmd.run(opt).await,
            Self::Register(cmd) => cmd.run(opt).await,
            Self::Status(cmd) => cmd.run(opt).await,
        }
    }
}

use std::path::PathBuf;

#[derive(Clone)]
pub struct KeystorePath(PathBuf);

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
