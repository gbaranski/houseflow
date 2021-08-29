use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;

pub(super) fn subcommand() -> clap::App<'static, 'static> {
    SubCommand::with_name("completions")
        .setting(AppSettings::Hidden)
        .about("Generate shell completions")
        .arg(
            Arg::with_name("shell")
                .help("Name of shell")
                .possible_values(&clap::Shell::variants())
                .takes_value(true),
        )
}
