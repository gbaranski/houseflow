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

mod context;

mod admin;
mod auth;

use anyhow::Context;
use context::{CommandContext, Tokens};

use async_trait::async_trait;
use std::str::FromStr;

pub(crate) fn dialoguer_theme() -> impl dialoguer::theme::Theme {
    dialoguer::theme::ColorfulTheme {
        ..dialoguer::theme::ColorfulTheme::default()
    }
}

#[async_trait]
pub trait Command {
    async fn run(self, ctx: CommandContext) -> anyhow::Result<()>;
}

use houseflow_config::client::Config;
use std::path::Path;

async fn main_async() -> anyhow::Result<()> {
    use clap::{App, AppSettings, Arg, SubCommand};

    houseflow_config::init_logging();
    let config_default_path = Config::default_path();

    let device_type_variants = houseflow_types::DeviceType::variants_string();
    let device_type_variants = device_type_variants
        .iter()
        .map(|e| e.as_ref())
        .collect::<Vec<&str>>();

    let device_trait_variants = houseflow_types::DeviceTrait::variants_string();
    let device_trait_variants = device_trait_variants
        .iter()
        .map(|e| e.as_ref())
        .collect::<Vec<&str>>();

    let admin_subcommand = SubCommand::with_name("admin")
        .about("Administrate the server, works only when user is admin")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("structure")
                .about("Manage structures")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add structures")
                        .arg(Arg::with_name("name").long("name").takes_value(true)),
                ),
        )
        .subcommand(
            SubCommand::with_name("room")
                .about("Manage rooms")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add rooms")
                        .arg(
                            Arg::with_name("structure-id")
                                .long("structure-id")
                                .takes_value(true),
                        )
                        .arg(Arg::with_name("name").long("name").takes_value(true)),
                ),
        )
        .subcommand(
            SubCommand::with_name("device")
                .about("Manage devices")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add devices")
                        .arg(
                            Arg::with_name("room-id")
                                .help("ID of the room to which the device belongs")
                                .long("room-id"),
                        )
                        .arg(
                            Arg::with_name("password")
                                .help("Password used to authenticate the device")
                                .long("password"),
                        )
                        .arg(
                            Arg::with_name("type")
                                .help("Type of the device, e.g Light")
                                .long("type")
                                .possible_values(&device_type_variants),
                        )
                        .arg(
                            Arg::with_name("trait")
                                .help("Trait that the device has")
                                .long("trait")
                                .multiple(true)
                                .possible_values(&device_trait_variants),
                        )
                        .arg(
                            Arg::with_name("name")
                                .help("Name of the device")
                                .long("name"),
                        )
                        .arg(
                            Arg::with_name("will-push-state")
                                .help("If present, device will push state, instead polling")
                                .long("will-push-state"),
                        )
                        .arg(
                            Arg::with_name("model")
                                .help("Model of the device")
                                .long("model"),
                        )
                        .arg(
                            Arg::with_name("hw-version")
                                .help("Hardware version of the device")
                                .long("hw-version"),
                        )
                        .arg(
                            Arg::with_name("sw-version")
                                .help("Hardware version of the device")
                                .long("sw-version"),
                        )
                        .arg(
                            Arg::with_name("attributes")
                                .help("Additional attributes of the device, in JSON format")
                                .long("attributes"),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("user-structure")
                .about("Manage user structures")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add user structures")
                        .arg(
                            Arg::with_name("structure-id")
                                .long("structure-id")
                                .takes_value(true),
                        )
                        .arg(Arg::with_name("user-id").long("user-id").takes_value(true))
                        .arg(Arg::with_name("is-manager").long("manager")),
                ),
        );

    let auth_subcommand = SubCommand::with_name("auth")
        .about("Login, Register, Logout, and refresh your authentication")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("login")
                .about("Log in with a Houseflow account")
                .arg(
                    Arg::with_name("email")
                        .help("Email used to log in, if not defined it will ask at runtime"),
                )
                .arg(
                    Arg::with_name("password")
                        .help("Password used to log in, if not defined it will ask at runtime"),
                ),
        )
        .subcommand(SubCommand::with_name("register").about("Register a Houseflow account"))
        .subcommand(SubCommand::with_name("logout").about("Log out from currently logged account"))
        .subcommand(
            SubCommand::with_name("refresh").about("Refresh stored authentication credentials"),
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("View authentication status")
                .arg(
                    Arg::with_name("show-token")
                        .help("Display the secret token in status")
                        .long("show-token"),
                ),
        );

    let matches = App::new("Houseflow")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Client for the Houseflow project")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .help("Configuration path")
                .default_value(config_default_path.to_str().unwrap()),
        )
        .subcommand(admin_subcommand)
        .subcommand(auth_subcommand)
        .get_matches();

    fn unwrap_subcommand<'a>(
        (name, matches): (&'a str, Option<&'a clap::ArgMatches>),
    ) -> (&'a str, &'a clap::ArgMatches<'a>) {
        (name, matches.unwrap())
    }

    fn get_input(prompt: impl Into<String>) -> String {
        dialoguer::Input::with_theme(&dialoguer_theme())
            .with_prompt(prompt)
            .interact_text()
            .unwrap()
    }

    fn get_inputs_with_variants(prompt: impl Into<String>, variants: &[&str]) -> Vec<String> {
        let prompt: String = prompt.into();
        std::iter::repeat_with(|| {
            dialoguer::Input::<String>::with_theme(&dialoguer_theme())
                .with_prompt(prompt.clone() + " (press ENTER to skip)")
                .allow_empty(true)
                .validate_with(|input: &String| {
                    if variants.contains(&input.as_str()) {
                        Ok(())
                    } else {
                        Err("Matching variant not found")
                    }
                })
                .interact_text()
                .unwrap()
        })
        .take_while(|s| s.trim() != "")
        .collect()
    }

    fn get_input_with_variants(prompt: impl Into<String>, variants: &[&str]) -> String {
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

    fn get_password(prompt: impl Into<String>) -> String {
        dialoguer::Password::with_theme(&dialoguer_theme())
            .with_prompt(prompt)
            .interact()
            .unwrap()
    }

    let subcommand = unwrap_subcommand(matches.subcommand());
    let config_path = Path::new(matches.value_of("config").unwrap());
    let ctx = CommandContext::new(config_path).await?;

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

    match subcommand {
        ("admin", matches) => match unwrap_subcommand(matches.subcommand()) {
            ("structure", matches) => match unwrap_subcommand(matches.subcommand()) {
                ("add", matches) => {
                    let structure_name = get_value(matches, get_input, "name")?;

                    admin::structure::add::Command { structure_name }.run(ctx)
                }
                _ => unreachable!(),
            },
            ("room", matches) => match unwrap_subcommand(matches.subcommand()) {
                ("add", matches) => admin::room::add::Command {
                    room_name: get_value(matches, get_input, "name")?,
                    structure_id: get_value(matches, get_input, "structure-id")?,
                }
                .run(ctx),
                _ => unreachable!(),
            },
            ("device", matches) => match unwrap_subcommand(matches.subcommand()) {
                ("add", matches) => admin::device::add::Command {
                    room_id: get_value(matches, get_input, "room-id")?,
                    password: get_value(matches, get_password, "password")?,
                    device_type: get_value(
                        matches,
                        |s| get_input_with_variants(s, &device_type_variants),
                        "type",
                    )?,
                    traits: get_values(
                        matches,
                        |s| get_inputs_with_variants(s, &device_trait_variants),
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
                            .unwrap_or_else(|| get_input("Attributes(in JSON)")),
                    )?,
                }
                .run(ctx),
                _ => unreachable!(),
            },
            ("user-structure", matches) => match unwrap_subcommand(matches.subcommand()) {
                ("add", matches) => admin::user_structure::add::Command {
                    structure_id: get_value(matches, get_input, "structure-id")?,
                    user_id: get_value(matches, get_input, "user-id")?,
                    is_manager: matches.is_present("manager"),
                }
                .run(ctx),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        ("auth", matches) => match unwrap_subcommand(matches.subcommand()) {
            ("login", matches) => auth::login::Command {
                email: get_value(matches, get_input, "email")?,
                password: get_value(matches, get_password, "password")?,
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
        _ => unreachable!(),
    }
    .await?;
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
