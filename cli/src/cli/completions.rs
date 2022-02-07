use clap::AppSettings;
use clap_complete::Shell;
use clap::Arg;

pub(super) fn subcommand() -> clap::App<'static> {
    clap::App::new("completions")
        .setting(AppSettings::Hidden)
        .about("Generate shell completions")
        .arg(
            Arg::new("shell")
                .help("Name of shell")
                .possible_values(Shell::possible_values())
                .takes_value(true),
        )
}
