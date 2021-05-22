use houseflow_types::{Device, DeviceID, User, UserID};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error when running migrations: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("Error when sending query: {0}")]
    QueryError(#[from] sqlx::Error),
}

pub struct Options {
    pub host: String,
    pub port: u16,
    pub password: String,
}

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(opts: Options) -> Self {
        let connect_options = PgConnectOptions::new()
            .host(&opts.host)
            .password(&opts.password)
            .port(opts.port);
        let pool_options = PgPoolOptions::new().max_connections(5);
        let pool = pool_options.connect_lazy_with(connect_options);
        Self { pool }
    }

    #[must_use = "you must initialise the database"]
    pub async fn init(&self) -> Result<(), Error> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        Ok(())
    }

    pub async fn get_user_by_id(&self, user_id: UserID) -> Result<Option<User>, Error> {
        const QUERY: &str = "SELECT * FROM users WHERE id = $1";

        let user_id: &str = &user_id.to_string();
        let user = sqlx::query_as::<_, User>(QUERY)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    pub async fn get_device_by_id(&self, device_id: DeviceID) -> Result<Option<Device>, Error> {
        const QUERY: &str = "SELECT * FROM devices WHERE id = $1";

        let device_id: &str = &device_id.to_string();
        let device = sqlx::query_as::<sqlx::Postgres, Device>(QUERY)
            .bind(device_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(device)
    }
}
