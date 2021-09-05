mod cli;
mod context;

mod auth;
mod fulfillment;

use anyhow::Context;
use context::CommandContext;
use context::Tokens;

use async_trait::async_trait;
use cli::get_input;
use cli::get_input_with_variants;
use cli::unwrap_subcommand;
use houseflow_types::code::VerificationCode;
use houseflow_types::DeviceCommand;
use std::str::FromStr;
use strum::VariantNames;

#[async_trait]
pub trait Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()>;
}

use houseflow_config::client::Config;
use houseflow_config::Config as _;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    houseflow_config::init_logging(true);
    let config_default_path = Config::default_path();
    let config_default_path = config_default_path.to_str().unwrap();
    let matches = cli::app(config_default_path).get_matches();
    let subcommand = unwrap_subcommand(matches.subcommand());
    let config_path = Path::new(matches.value_of("config").unwrap());
    let ctx = CommandContext::new(config_path.to_path_buf()).await?;

    match subcommand {
        ("auth", matches) => match unwrap_subcommand(matches.subcommand()) {
            ("login", matches) => {
                auth::login::Command {
                    email: get_value(matches, get_input, "email")?,
                    code: matches
                        .value_of("code")
                        .map(|str| VerificationCode::from_str(str).unwrap()),
                }
                .run(ctx)
                .await
            }
            ("logout", _) => auth::logout::Command {}.run(ctx).await,
            ("refresh", _) => auth::refresh::Command {}.run(ctx).await,
            ("status", matches) => {
                auth::status::Command {
                    show_token: matches.is_present("show-token"),
                }
                .run(ctx)
                .await
            }
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
                fulfillment::execute::Command {
                    device_id: get_value(matches, get_input, "device-id")?,
                    command: get_value(
                        matches,
                        |s| get_input_with_variants(s, DeviceCommand::VARIANTS),
                        "command",
                    )?,
                    params: serde_json::from_str(
                        &matches
                            .value_of("params")
                            .map(std::string::ToString::to_string)
                            .unwrap(),
                    )?,
                }
                .run(ctx)
                .await
            }
            ("query", matches) => {
                fulfillment::query::Command {
                    device_id: get_value(matches, get_input, "device-id")?,
                }
                .run(ctx)
                .await
            }
            ("sync", _) => fulfillment::sync::Command {}.run(ctx).await,
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

// fn get_values<T, TE, AF>(
//     matches: &clap::ArgMatches,
//     alt: AF,
//     name: &'static str,
// ) -> anyhow::Result<Vec<T>>
// where
//     AF: FnOnce(String) -> Vec<String>,
//     T: FromStr<Err = TE>,
//     TE: Into<anyhow::Error>,
// {
//     use inflector::Inflector;
//     matches
//         .values_of(name)
//         .map(|e| e.map(std::string::ToString::to_string).collect::<Vec<_>>())
//         .unwrap_or_else(|| alt(name.to_title_case()))
//         .iter()
//         .map(|v| {
//             T::from_str(v)
//                 .map_err(|err| -> anyhow::Error { err.into() })
//                 .with_context(|| name.to_title_case())
//         })
//         .collect::<Result<Vec<_>, _>>()
// }
