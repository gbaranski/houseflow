use clap::{App, AppSettings, Arg, SubCommand};
use houseflow_types::{DeviceCommand, DeviceTrait, DeviceType};
use strum::VariantNames;

pub(crate) fn dialoguer_theme() -> impl dialoguer::theme::Theme {
    dialoguer::theme::ColorfulTheme {
        ..dialoguer::theme::ColorfulTheme::default()
    }
}

pub fn app<'a>(default_config_path: &'a str) -> clap::App<'a, 'a> {
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
                                .possible_values(DeviceType::VARIANTS),
                        )
                        .arg(
                            Arg::with_name("trait")
                                .help("Trait that the device has")
                                .long("trait")
                                .multiple(true)
                                .possible_values(DeviceTrait::VARIANTS),
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

    let fulfillment_command = SubCommand::with_name("fulfillment")
        .about("Send Sync, Query, Execute intents to fulfillment service")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("sync").about("Sync devices"))
        .subcommand(
            SubCommand::with_name("query")
                .about("Query device state")
                .arg(
                    Arg::with_name("device-id")
                        .help("ID of the device to be queried")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("execute")
                .about("Execute command on device")
                .arg(
                    Arg::with_name("device-id")
                        .help("ID of the device to be queried")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("command")
                        .help("Command to be executed on the device")
                        .required(true)
                        .takes_value(true)
                        .possible_values(DeviceCommand::VARIANTS),
                )
                .arg(
                    Arg::with_name("params")
                        .help("Parameters of the execute request in JSON format")
                        .default_value("{}")
                        .takes_value(true)
                        .validator(|s| match serde_json::from_str::<serde_json::Value>(&s) {
                            Ok(_) => Ok(()),
                            Err(err) => Err(err.to_string()),
                        }),
                ),
        );

    let completions_subcommand = SubCommand::with_name("completions")
        .setting(AppSettings::Hidden)
        .about("Generate shell completions")
        .arg(
            Arg::with_name("shell")
                .help("Name of shell")
                .possible_values(&clap::Shell::variants())
                .takes_value(true),
        );

    App::new("Houseflow")
        .bin_name(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Client for the Houseflow project")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("config")
                .short("c")
                .help("Configuration path")
                .default_value(default_config_path),
        )
        .subcommand(admin_subcommand)
        .subcommand(auth_subcommand)
        .subcommand(fulfillment_command)
        .subcommand(completions_subcommand)
}

pub fn unwrap_subcommand<'a>(
    (name, matches): (&'a str, Option<&'a clap::ArgMatches>),
) -> (&'a str, &'a clap::ArgMatches<'a>) {
    (name, matches.unwrap())
}

pub fn get_input(prompt: impl Into<String>) -> String {
    dialoguer::Input::with_theme(&dialoguer_theme())
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

pub fn get_inputs_with_variants(prompt: impl Into<String>, variants: &[&str]) -> Vec<String> {
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

pub fn get_password(prompt: impl Into<String>) -> String {
    dialoguer::Password::with_theme(&dialoguer_theme())
        .with_prompt(prompt)
        .interact()
        .unwrap()
}
