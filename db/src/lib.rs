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
    Other(String),
    MissingEnv{
        key: &'static str,
    },
    PoolError(deadpool_postgres::PoolError),
    PgError(tokio_postgres::Error)

}

impl From<deadpool_postgres::PoolError> for Error {
    fn from(err: deadpool_postgres::PoolError) -> Error {
        Error::PoolError(err)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(err: tokio_postgres::Error) -> Error {
        Error::PgError(err)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(err) => write!(f, "Other: `{}`", err),
            Error::MissingEnv{key} => write!(f, "Missing `{}` enviroment variable", key),
            Error::PoolError(err) => write!(f, "Pool: `{}`", err),
            Error::PgError(err) => write!(f, "PG: `{}`", err),
        }
    }
}

fn read_env(key: &'static str) -> Result<String, Error> {
    std::env::var(key)
        .map_err(|err| match err {
            std::env::VarError::NotPresent => Error::MissingEnv{key},
            std::env::VarError::NotUnicode(err) => Error::Other(format!("`{}` is not valid unicode `{:?}`", key, err)),
        })
}

#[derive(Clone)]
pub struct Database {
    pool: deadpool_postgres::Pool,
}


impl Database {
    pub fn connect() -> Result<Database, Error> {
        let cfg = deadpool_postgres::Config {
            user: Some(read_env("POSTGRES_USER")?),
            password: Some(read_env("POSTGRES_PASSWORD")?),
            dbname: Some(read_env("POSTGRES_DB")?),
            options: None,
            application_name: None,
            ssl_mode: None,
            host: Some(read_env("POSTGRES_HOST")?),
            hosts: None,
            port: Some(read_env("POSTGRES_PORT")?)
                .map(|p| u16::from_str_radix(&p, 10)
                .expect("`POSTGRES_PORT` is invalid unsigned 16 bit integer")),
            ports: None,
            connect_timeout: None,
            keepalives: None,
            keepalives_idle: None,
            target_session_attrs: None,
            channel_binding: None,
            manager: None,
            pool: None,
        };

        let pool = cfg.create_pool(tokio_postgres::NoTls).unwrap();

        Ok(Database{
            pool,
        })

    }

    pub async fn init(&self) -> Result<(), Error> {
        let client = self.pool.get().await?;

        log::debug!("Creating `users` table");
        client.batch_execute(models::USER_SCHEMA).await?;
        log::debug!("Creating `devices` table");
        client.batch_execute(models::DEVICE_SCHEMA).await?;
        log::debug!("Creating `user_devices` table");
        client.batch_execute(models::USER_DEVICES_SCHEMA).await?;

        Ok(())
    }
}
