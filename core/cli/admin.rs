use clap::{AppSettings, Arg, SubCommand};
use houseflow_types::{DeviceTrait, DeviceType};
use strum::VariantNames;

fn structure_add() -> clap::App<'static, 'static> {
    SubCommand::with_name("add")
        .about("Add structures")
        .arg(Arg::with_name("name").long("name").takes_value(true))
}

fn structure() -> clap::App<'static, 'static> {
    SubCommand::with_name("structure")
        .about("Manage structures")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(structure_add())
}

fn room_add() -> clap::App<'static, 'static> {
    SubCommand::with_name("add")
        .about("Add rooms")
        .arg(
            Arg::with_name("structure-id")
                .long("structure-id")
                .takes_value(true),
        )
        .arg(Arg::with_name("name").long("name").takes_value(true))
}

fn room() -> clap::App<'static, 'static> {
    SubCommand::with_name("room")
        .about("Manage rooms")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(room_add())
}

fn device_add() -> clap::App<'static, 'static> {
    SubCommand::with_name("add")
        .about("Add devices")
        .arg(
            Arg::with_name("room-id")
                .help("ID of the room to which the device belongs")
                .long("room-id")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("password")
                .help("Password used to authenticate the device")
                .long("password")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("type")
                .help("Type of the device, e.g Light")
                .long("type")
                .possible_values(DeviceType::VARIANTS)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("trait")
                .help("Trait that the device has")
                .long("trait")
                .multiple(true)
                .possible_values(DeviceTrait::VARIANTS)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("name")
                .help("Name of the device")
                .long("name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("will-push-state")
                .help("If present, device will push state, instead polling")
                .long("will-push-state")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("model")
                .help("Model of the device")
                .long("model")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("hw-version")
                .help("Hardware version of the device")
                .long("hw-version")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sw-version")
                .help("Hardware version of the device")
                .long("sw-version")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("attributes")
                .help("Additional attributes of the device, in JSON format")
                .default_value("{}")
                .long("attributes")
                .validator(super::validate_json)
                .takes_value(true),
        )
}

fn device() -> clap::App<'static, 'static> {
    SubCommand::with_name("device")
        .about("Manage devices")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(device_add())
}

fn user_structure_add() -> clap::App<'static, 'static> {
    SubCommand::with_name("add")
        .about("Add user structures")
        .arg(
            Arg::with_name("structure-id")
                .long("structure-id")
                .takes_value(true),
        )
        .arg(Arg::with_name("user-id").long("user-id").takes_value(true))
        .arg(Arg::with_name("is-manager").long("manager"))
}

fn user_structure() -> clap::App<'static, 'static> {
    SubCommand::with_name("user-structure")
        .about("Manage user structures")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(user_structure_add())
}

pub(super) fn subcommand() -> clap::App<'static, 'static> {
    SubCommand::with_name("admin")
        .about("Administrate the server, works only when user is admin")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(structure())
        .subcommand(room())
        .subcommand(device())
        .subcommand(user_structure())
}
