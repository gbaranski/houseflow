use clap::{AppSettings, Arg, SubCommand};

fn login() -> clap::App<'static, 'static> {
    SubCommand::with_name("login")
        .about("Log in with a Houseflow account")
        .arg(
            Arg::with_name("email")
                .help("Email used to log in, if not defined it will ask at runtime"),
        )
        .arg(
            Arg::with_name("password")
                .help("Password used to log in, if not defined it will ask at runtime"),
        )
}

fn register() -> clap::App<'static, 'static> {
    SubCommand::with_name("register").about("Register a Houseflow account")
}

fn logout() -> clap::App<'static, 'static> {
    SubCommand::with_name("logout").about("Log out from currently logged account")
}

fn refresh() -> clap::App<'static, 'static> {
    SubCommand::with_name("refresh").about("Refresh stored authentication credentials")
}

fn status() -> clap::App<'static, 'static> {
    SubCommand::with_name("status")
        .about("View authentication status")
        .arg(
            Arg::with_name("show-token")
                .help("Display the secret token in status")
                .long("show-token"),
        )
}

pub(super) fn subcommand() -> clap::App<'static, 'static> {
    SubCommand::with_name("auth")
        .about("Login, Register, Logout, and refresh your authentication")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(login())
        .subcommand(register())
        .subcommand(logout())
        .subcommand(refresh())
        .subcommand(status())
}
