// mod cli;
// mod config;
//
// mod admin;
// mod auth;
// mod fulfillment;
//
// mod context;
//
// use context::CommandContext;
//
// use async_trait::async_trait;
// use cli::{CliConfig, Subcommand};
//
//

mod cli;
mod context;

mod admin;
mod auth;
mod fulfillment;

use anyhow::Context;
use context::{CommandContext, Tokens};

use async_trait::async_trait;
use cli::{
    get_input, get_input_with_variants, get_inputs_with_variants, get_password, unwrap_subcommand,
};
use houseflow_types::{DeviceCommand, DeviceTrait, DeviceType};
use std::str::FromStr;
use strum::VariantNames;

#[async_trait]
pub trait Command {
    async fn run(self, mut ctx: CommandContext) -> anyhow::Result<()>;
}

use houseflow_config::{client::Config, Config as _};
use std::path::Path;

async fn main_async() -> anyhow::Result<()> {
    houseflow_config::init_logging();
    let config_default_path = Config::default_path();
    let config_default_path = config_default_path.to_str().unwrap();
    let matches = cli::app(&config_default_path).get_matches();
    let subcommand = unwrap_subcommand(matches.subcommand());
    let config_path = Path::new(matches.value_of("config").unwrap());
    let ctx = CommandContext::new(config_path.to_path_buf()).await?;

    match subcommand {
        ("admin", matches) => match unwrap_subcommand(matches.subcommand()) {
            ("structure", matches) => match unwrap_subcommand(matches.subcommand()) {
                ("add", matches) => {
                    let structure_name = get_value(matches, get_input, "name")?;

                    admin::structure::add::Command { structure_name }
                        .run(ctx)
                        .await
                }
                _ => unreachable!(),
            },
            ("room", matches) => match unwrap_subcommand(matches.subcommand()) {
                ("add", matches) => {
                    admin::room::add::Command {
                        room_name: get_value(matches, get_input, "name")?,
                        structure_id: get_value(matches, get_input, "structure-id")?,
                    }
                    .run(ctx)
                    .await
                }
                _ => unreachable!(),
            },
            ("device", matches) => match unwrap_subcommand(matches.subcommand()) {
                ("add", matches) => {
                    admin::device::add::Command {
                        room_id: get_value(matches, get_input, "room-id")?,
                        password: get_value(matches, get_password, "password")?,
                        device_type: get_value(
                            matches,
                            |s| get_input_with_variants(s, DeviceType::VARIANTS),
                            "type",
                        )?,
                        traits: get_values(
                            matches,
                            |s| get_inputs_with_variants(s, DeviceTrait::VARIANTS),
                            "trait",
                        )?,
                        name: get_value(matches, get_input, "name")?,
                        will_push_state: matches.is_present("will-push-state"),
                        model: get_value(matches, get_input, "model")?,
                        hw_version: get_value(matches, get_input, "hw-version")?,
                        sw_version: get_value(matches, get_input, "sw-version")?,
                        attributes: serde_json::from_str(
                            &matches
                                .value_of("attributes")
                                .map(std::string::ToString::to_string)
                                .unwrap(),
                        )?,
                    }
                    .run(ctx)
                    .await
                }
                _ => unreachable!(),
            },
            ("user-structure", matches) => match unwrap_subcommand(matches.subcommand()) {
                ("add", matches) => {
                    admin::user_structure::add::Command {
                        structure_id: get_value(matches, get_input, "structure-id")?,
                        user_id: get_value(matches, get_input, "user-id")?,
                        is_manager: matches.is_present("manager"),
                    }
                    .run(ctx)
                    .await
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        ("auth", matches) => match unwrap_subcommand(matches.subcommand()) {
            ("login", matches) => {
                auth::login::Command {
                    email: get_value(matches, get_input, "email")?,
                    password: get_value(matches, get_password, "password")?,
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

fn main() -> anyhow::Result<()> {
    actix_rt::System::with_tokio_rt(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(async move { main_async().await })
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

fn get_values<T, TE, AF>(
    matches: &clap::ArgMatches,
    alt: AF,
    name: &'static str,
) -> anyhow::Result<Vec<T>>
where
    AF: FnOnce(String) -> Vec<String>,
    T: FromStr<Err = TE>,
    TE: Into<anyhow::Error>,
{
    use inflector::Inflector;
    matches
        .values_of(name)
        .map(|e| e.map(std::string::ToString::to_string).collect::<Vec<_>>())
        .unwrap_or_else(|| alt(name.to_title_case()))
        .iter()
        .map(|v| {
            T::from_str(v)
                .map_err(|err| -> anyhow::Error { err.into() })
                .with_context(|| name.to_title_case())
        })
        .collect::<Result<Vec<_>, _>>()
}
