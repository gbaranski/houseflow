use clap::Arg;
use clap::Command;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use std::str::FromStr;
use strum::VariantNames;

fn read() -> Command<'static> {
    Command::new("read")
        .about("Read characteristic of the accessory")
        .arg(
            Arg::new("accessory-id")
                .help("ID of the accessory")
                .long("accessory")
                .validator(|s| match accessory::ID::from_str(s) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err.to_string()),
                })
                .takes_value(true),
        )
        .arg(
            Arg::new("characteristic-name")
                .help("Name of the characteristic to read")
                .long("characteristic")
                .validator(|s| {
                    match CharacteristicName::VARIANTS
                        .iter()
                        .find(|v| v.to_string() == s)
                    {
                        Some(_) => Ok(()),
                        None => Err(format!(
                            "variant {} not found. Available variants: [{}]",
                            s,
                            CharacteristicName::VARIANTS.join(",")
                        )),
                    }
                })
                .takes_value(true),
        )
        .arg(
            Arg::new("service-name")
                .help("Name of the service to read")
                .long("service")
                .validator(
                    |s| match ServiceName::VARIANTS.iter().find(|v| v.to_string() == s) {
                        Some(_) => Ok(()),
                        None => Err(format!(
                            "variant {} not found. Available variants: [{}]",
                            s,
                            ServiceName::VARIANTS.join(",")
                        )),
                    },
                )
                .takes_value(true),
        )
}

pub(super) fn subcommand() -> Command<'static> {
    Command::new("meta")
        .about("Read or write characteristic of the accessory")
        .subcommand(read())
        .subcommand_required(true)
        .arg_required_else_help(true)
}
