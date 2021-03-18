pub mod models {
    mod user;
    mod device;

    pub use user::User;
    pub use device::Device;

    pub(crate) use user::USER_SCHEMA;
    pub(crate) use user::USER_DEVICES_SCHEMA;
    pub(crate) use device::DEVICE_SCHEMA;
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

impl From<deadpool_postgres::PoolError> for Error {
    fn from(err: deadpool_postgres::PoolError) -> Error {
        err.into()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone)]
pub struct Database {
    pool: deadpool_postgres::Pool,
}

fn read_env(key: &'static str) -> Result<String, Error> {
    use std::env::var;
    var(key)
        .map_err(|err| Error::Error(format!("fail reading `{}`: `{}`", key, err)))
    
}

impl Database {
    pub async fn connect() -> Result<Database, Error> {

        let config = deadpool_postgres::Config {
            user: Some(read_env("POSTGRES_USER")?),
            password: Some(read_env("POSTGRES_PASSWORD")?),
            dbname: Some(read_env("POSTGRES_DB")?),
            options: None,
            application_name: Some("houseflow".to_string()),
            ssl_mode: None,
            host: Some(read_env("POSTGRES_HOST")?),
            hosts: None,
            port: Some(5432),
            ports: None,
            connect_timeout: None,
            keepalives: None,
            keepalives_idle: None,
            target_session_attrs: None,
            channel_binding: None,
            manager: Some(deadpool_postgres::ManagerConfig{
                recycling_method: deadpool_postgres::RecyclingMethod::Fast,
            }),
            pool: None
        };

        let pool = config.create_pool(tokio_postgres::NoTls).unwrap();
        let client = pool.get().await?;

        client.batch_execute(models::USER_SCHEMA).await?;
        client.batch_execute(models::USER_DEVICES_SCHEMA).await?;
        client.batch_execute(models::DEVICE_SCHEMA).await?;

        Ok(Database{
            pool,
        })
    }
}
