use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;
use houseflow_types::DeviceCommand;
use strum::VariantNames;

fn sync() -> clap::App<'static, 'static> {
    SubCommand::with_name("sync").about("Sync devices")
}

fn query() -> clap::App<'static, 'static> {
    SubCommand::with_name("query")
        .about("Query device state")
        .arg(
            Arg::with_name("device-id")
                .help("ID of the device to be queried")
                .required(true)
                .takes_value(true),
        )
}

fn execute() -> clap::App<'static, 'static> {
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
                .help("Name of command to be executed on the device")
                .long("command")
                .required(true)
                .takes_value(true)
                .possible_values(DeviceCommand::VARIANTS),
        )
        .arg(
            Arg::with_name("params")
                .help("Parameters of the execute request in JSON format")
                .long("params")
                .default_value("{}")
                .takes_value(true)
                .validator(super::validate_json),
        )
}

pub(super) fn subcommand() -> clap::App<'static, 'static> {
    SubCommand::with_name("fulfillment")
        .about("Send Sync, Query, Execute intents to fulfillment service")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(sync())
        .subcommand(query())
        .subcommand(execute())
}
