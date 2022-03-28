use clap::Arg;
use clap::Command;
use clap_complete::Shell;

pub(super) fn subcommand() -> Command<'static> {
    Command::new("completions")
        .about("Generate shell completions")
        .arg(
            Arg::new("shell")
                .help("Name of shell")
                .possible_values(Shell::possible_values())
                .takes_value(true),
        )
        .hide(true)
}
