use crate::Opt;
use structopt::StructOpt;

const AUTH_PORT: u16 = 6001;

#[derive(StructOpt)]
pub enum Service {
    Auth,
}

#[derive(StructOpt)]
pub struct RunCommand {
    #[structopt(subcommand)]
    pub service: Service,
}

pub async fn run(opt: &Opt, command: &RunCommand) -> anyhow::Result<()> {
    match command.service {
        Service::Auth => run_auth(opt).await,
    }
}


async fn run_auth(opt: &Opt) -> anyhow::Result<()> {
    let address = opt
        .auth_url
        .socket_addrs(|| Some(AUTH_PORT))
        .expect("invalid address");

    let address = address.first().unwrap();

    let token_store = houseflow_auth_server::MemoryTokenStore::new();
    let database = houseflow_db::MemoryDatabase::new();
    let app_data = houseflow_auth_server::AppData {
        refresh_key: Vec::from("some-refresh-key"),
        access_key: Vec::from("some-access-key"),
        password_salt: Vec::from("some-password-salt"),
    };
    houseflow_auth_server::run(address, token_store, database, app_data).await?;
    Ok(())
}
