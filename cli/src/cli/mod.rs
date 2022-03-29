mod auth;
mod completions;
mod meta;

use clap::Arg;
use clap::Command;

pub(crate) fn dialoguer_theme() -> impl dialoguer::theme::Theme {
    dialoguer::theme::ColorfulTheme {
        ..dialoguer::theme::ColorfulTheme::default()
    }
}

pub fn app(default_config_path: &'static std::ffi::OsStr) -> Command<'_> {
    Command::new("Houseflow")
        .bin_name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Client for the Houseflow project")
        .arg(
            Arg::new("config")
                .short('c')
                .help("Configuration path")
                .default_value_os(default_config_path),
        )
        .subcommand(auth::subcommand())
        .subcommand(meta::subcommand())
        .subcommand(completions::subcommand())
        .subcommand_required(true)
        .arg_required_else_help(true)
}

pub fn get_input(prompt: impl Into<String>) -> String {
    dialoguer::Input::with_theme(&dialoguer_theme())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}
