use clap::AppSettings;
use clap::Arg;
use clap::SubCommand;
use houseflow_types::code::VerificationCode;
use std::str::FromStr;

fn login() -> clap::App<'static, 'static> {
    SubCommand::with_name("login")
        .about("Log in with a Houseflow account")
        .arg(
            Arg::with_name("email")
                .help("Email used to log in")
                .long("email")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("code")
                .help("Verification code")
                .long("code")
                .validator(|s| match VerificationCode::from_str(&s) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err.to_string()),
                })
                .takes_value(true),
        )
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
        .about("Login, Logout, and refresh your authentication")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(login())
        .subcommand(logout())
        .subcommand(refresh())
        .subcommand(status())
}
