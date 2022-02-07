mod auth;
mod cli;
mod context;
mod meta;

use anyhow::Context;
use async_trait::async_trait;
use cli::get_input;
use context::CommandContext;
use context::Tokens;
use houseflow_config::client::Config;
use houseflow_config::Config as _;
use houseflow_types::code::VerificationCode;
use lazy_static::lazy_static;
use std::path::Path;
use std::str::FromStr;

lazy_static! {
    static ref DEFAULT_CONFIG_PATH: std::path::PathBuf = Config::default_path();
}

#[async_trait]
pub trait Command {
    async fn run(self, ctx: CommandContext) -> anyhow::Result<()>;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    houseflow_config::log::init();
    let matches = cli::app(DEFAULT_CONFIG_PATH.as_os_str()).get_matches();
    let subcommand = matches.subcommand().unwrap();
    let config_path = Path::new(matches.value_of("config").unwrap());
    let ctx = CommandContext::new(config_path.to_path_buf())?;

    match subcommand {
        ("auth", matches) => match matches.subcommand().unwrap() {
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
        ("meta", matches) => match matches.subcommand().unwrap() {
            ("read", matches) => {
                meta::read::Command {
                    accessory_id: get_value(matches, get_input, "accessory-id")?,
                    service_name: get_value(matches, get_input, "service-name")?,
                    characteristic_name: get_value(matches, get_input, "characteristic-name")?,
                }
                .run(ctx)
                .await
            }
            _ => unreachable!(),
        },
        ("completions", matches) => {
            use clap_complete::Shell;
            let mut app = cli::app(DEFAULT_CONFIG_PATH.as_os_str());
            let shell = matches.value_of("shell").unwrap();
            let shell = Shell::from_str(shell).unwrap();
            let bin_name = app.get_bin_name().unwrap().to_owned();
            clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
            Ok(())
        }
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
