#[cfg(feature = "client")]
pub(crate) fn get_dialoguer_theme() -> impl dialoguer::theme::Theme {
    dialoguer::theme::ColorfulTheme {
        ..dialoguer::theme::ColorfulTheme::default()
    }
}

use clap::Clap;

#[derive(Clap)]
pub enum Subcommand {
    #[cfg(feature = "client")]
    /// Login, register, logout, and refresh your authentication
    Auth(crate::AuthCommand),

    #[cfg(feature = "client")]
    /// Manage the fulfillment service, sync devices, execute command, query state
    Fulfillment(crate::FulfillmentCommand),

    #[cfg(feature = "server")]
    /// Login, register, logout, and refresh your authentication
    Server(crate::ServerCommand),

    #[cfg(feature = "device")]
    /// Run, manage device
    Device(crate::DeviceCommand),

    /// Manage configuration(s)
    Config(crate::ConfigCommand),
}

use std::path::PathBuf;

#[derive(Clap)]
pub struct CliConfig {
    #[clap(subcommand)]
    pub subcommand: Subcommand,

    /// Config directory, server.toml, client.toml, device.toml will be used. Default: $XDG_CONFIG_HOME/houseflow
    #[clap(long)]
    pub config_directory: Option<PathBuf>,
}
