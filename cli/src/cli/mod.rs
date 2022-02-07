mod auth;
mod completions;
mod meta;

use clap::App;
use clap::AppSettings;
use clap::Arg;

pub(crate) fn dialoguer_theme() -> impl dialoguer::theme::Theme {
    dialoguer::theme::ColorfulTheme {
        ..dialoguer::theme::ColorfulTheme::default()
    }
}

fn validate_json(s: &str) -> Result<(), String> {
    match serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(s) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub fn app(default_config_path: &'static std::ffi::OsStr) -> clap::App<'_> {
    App::new("Houseflow")
        .bin_name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Client for the Houseflow project")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::new("config")
                .short('c')
                .help("Configuration path")
                .default_value_os(default_config_path)
        )
        .subcommand(auth::subcommand())
        .subcommand(meta::subcommand())
        .subcommand(completions::subcommand())
}

pub fn get_input(prompt: impl Into<String>) -> String {
    dialoguer::Input::with_theme(&dialoguer_theme())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

pub fn get_input_with_variants(prompt: impl Into<String>, variants: &[&str]) -> String {
    dialoguer::Input::with_theme(&dialoguer_theme())
        .with_prompt(prompt)
        .validate_with(|input: &String| {
            if variants.contains(&input.as_str()) {
                Ok(())
            } else {
                Err("Matching variant not found")
            }
        })
        .interact_text()
        .unwrap()
}
