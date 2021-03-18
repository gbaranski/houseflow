pub mod device;
pub mod user;

pub enum Error {
    Error(String),
    PgError(tokio_postgres::Error)
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Error {
        err.into()
    }
}

pub struct DatabaseOptions {
    pub user: String,
    pub password: String,
    pub host: String,
    pub db_name: String,
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
