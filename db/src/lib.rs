pub mod models {
    mod user;
    mod device;

    pub use user::*;
    pub use device::*;
}

#[derive(Debug)]
pub enum Error {
    Error(String),
    PgError(tokio_postgres::Error)
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Error {
        err.into()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// impl std::string::ToString for Error {
//     fn to_string(&self) -> String {
//         self
//     }
// }

pub struct DatabaseOptions {
    pub user: String,
    pub password: String,
    pub host: String,
    pub db_name: String,
}

impl DatabaseOptions {
    /// Retrieves DatabaseOptions from enviroment variables
    pub fn from_env() -> Result<DatabaseOptions, String> {
        use std::env::var;

        Ok(DatabaseOptions {
            user: var("POSTGRES_USER")
                .map_err(|err| format!("fail loading `POSTGRES_USER`: `{}`", err))?,
            password: var("POSTGRES_PASSWORD")
                .map_err(|err| format!("fail loading `POSTGRES_PASSWORD`: `{}`", err))?,
            host: var("POSTGRES_HOST")
                .map_err(|err| format!("fail loading `POSTGRES_HOST`: `{}`", err))?,
            db_name: var("POSTGRES_DB")
                .map_err(|err| format!("fail loading `POSTGRES_DB`: `{}`", err))?,
        })
    }
}

pub struct Database {
    client: tokio_postgres::Client,
    options: DatabaseOptions
}

impl Database {
    pub async fn connect(options: DatabaseOptions) -> Result<Database, Error> {
        let cstr = format!("user={} password={} host={} db={}", 
                           options.user, 
                           options.password, 
                           options.host, 
                           options.db_name
                           );

        let (client, _connection) = 
            tokio_postgres::connect(cstr.as_ref(), tokio_postgres::NoTls).await?;

        client.batch_execute(device::DEVICE_SCHEMA).await?;

        Ok(Database{
            client,
            options,
        })
    }
}
