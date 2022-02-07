use clap::AppSettings;
use clap::Arg;
use houseflow_types::code::VerificationCode;
use std::str::FromStr;

fn login() -> clap::App<'static> {
    clap::App::new("login")
        .about("Log in with a Houseflow account")
        .arg(
            Arg::new("email")
                .help("Email used to log in")
                .long("email")
                .takes_value(true),
        )
        .arg(
            Arg::new("code")
                .help("Verification code")
                .long("code")
                .validator(|s| match VerificationCode::from_str(&s) {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err.to_string()),
                })
                .takes_value(true),
        )
}

fn logout() -> clap::App<'static> {
    clap::App::new("logout").about("Log out from currently logged account")
}

fn refresh() -> clap::App<'static> {
    clap::App::new("refresh").about("Refresh stored authentication credentials")
}

fn status() -> clap::App<'static> {
    clap::App::new("status")
        .about("View authentication status")
        .arg(
            Arg::new("show-token")
                .help("Display the secret token in status")
                .long("show-token"),
        )
}

pub(super) fn subcommand() -> clap::App<'static> {
    clap::App::new("auth")
        .about("Login, Logout, and refresh your authentication")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(login())
        .subcommand(logout())
        .subcommand(refresh())
        .subcommand(status())
}
