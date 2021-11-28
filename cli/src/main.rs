mod cli;
mod context;

mod auth;
mod fulfillment;

use anyhow::Context;
use cli::get_input;
use cli::get_input_with_variants;
use cli::unwrap_subcommand;
use context::CommandContext;
use context::Tokens;
use houseflow_config::client::Config;
use houseflow_config::Config as _;
use houseflow_types::code::VerificationCode;
use houseflow_types::accessory;
use std::path::Path;
use std::str::FromStr;
use strum::VariantNames;

pub trait Command {
    fn run(self, ctx: CommandContext) -> anyhow::Result<()>;
}

fn main() -> anyhow::Result<()> {
    houseflow_config::init_logging(true);
    let config_default_path = Config::default_path();
    let config_default_path = config_default_path.to_str().unwrap();
    let matches = cli::app(config_default_path).get_matches();
    let subcommand = unwrap_subcommand(matches.subcommand());
    let config_path = Path::new(matches.value_of("config").unwrap());
    let ctx = CommandContext::new(config_path.to_path_buf())?;

    match subcommand {
        ("auth", matches) => match unwrap_subcommand(matches.subcommand()) {
            ("login", matches) => auth::login::Command {
                email: get_value(matches, get_input, "email")?,
                code: matches
                    .value_of("code")
                    .map(|str| VerificationCode::from_str(str).unwrap()),
            }
            .run(ctx),
            ("logout", _) => auth::logout::Command {}.run(ctx),
            ("refresh", _) => auth::refresh::Command {}.run(ctx),
            ("status", matches) => auth::status::Command {
                show_token: matches.is_present("show-token"),
            }
            .run(ctx),
            _ => unreachable!(),
        },
        ("completions", matches) => {
            use clap::Shell;
            let mut app = cli::app(config_default_path);
            let shell = matches.value_of("shell").unwrap();
            let shell = Shell::from_str(shell).unwrap();
            let bin_name = app.get_bin_name().unwrap().to_owned();
            app.gen_completions_to(bin_name, shell, &mut std::io::stdout());
            Ok(())
        }
        ("fulfillment", matches) => match unwrap_subcommand(matches.subcommand()) {
            ("execute", matches) => {
                let json = serde_json::json!({
                    "command": get_value::<String, _, _>(matches, |s| get_input_with_variants(s, accessory::Command::VARIANTS), "command").context("get command")?,
                    "params": get_value::<serde_json::Value, _, _>(matches, get_input, "params").context("get params")?
                });
                let command = serde_json::from_value(json).context("parse into command")?;
                fulfillment::execute::Command {
                    device_id: get_value(matches, get_input, "device-id")?,
                    command,
                    params: serde_json::from_str(
                        &matches
                            .value_of("params")
                            .map(std::string::ToString::to_string)
                            .unwrap(),
                    )?,
                }
                .run(ctx)
            }
            ("query", matches) => fulfillment::query::Command {
                device_id: get_value(matches, get_input, "device-id")?,
            }
            .run(ctx),
            ("sync", _) => fulfillment::sync::Command {}.run(ctx),
            _ => todo!(),
        },
        _ => unreachable!(),
    }?;
    Ok::<(), anyhow::Error>(())
}

fn get_value<T, TE, AF>(
    matches: &clap::ArgMatches,
    alt: AF,
    name: &'static str,
) -> anyhow::Result<T>
where
    AF: FnOnce(String) -> String,
    T: FromStr<Err = TE>,
    TE: Into<anyhow::Error>,
{
    use inflector::Inflector;
    let str = matches
        .value_of(name)
        .map(std::string::ToString::to_string)
        .unwrap_or_else(|| alt(name.to_title_case()));

    T::from_str(&str)
        .map_err(|err| -> anyhow::Error { err.into() })
        .with_context(|| name.to_title_case())
}
