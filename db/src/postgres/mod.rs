use crate::{Database, DatabaseInternalError, Error};
use async_trait::async_trait;
use deadpool_postgres::Pool;
use semver::Version;
use tokio_postgres::NoTls;
use types::{Device, DeviceID, User, UserID};

use refinery::embed_migrations;
embed_migrations!("migrations");

#[derive(Debug, thiserror::Error)]
pub enum PostgresError {
    #[error("Error when sending query: `{0}`")]
    QueryError(#[from] tokio_postgres::Error),

    #[error("pool error: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("Column `{column}` is invalid: `{error}`")]
    InvalidColumn {
        column: &'static str,
        error: Box<dyn std::error::Error>,
    },

    #[error("Error when running migrations: `{0}`")]
    MigrationError(#[from] refinery::Error),
}

impl DatabaseInternalError for PostgresError {}
impl DatabaseInternalError for deadpool_postgres::PoolError {}
impl DatabaseInternalError for tokio_postgres::Error {}
impl DatabaseInternalError for refinery::Error {}

#[derive(Clone)]
pub struct PostgresDatabase {
    pool: Pool<NoTls>,
}

pub struct PostgresConfig<'a> {
    pub user: &'a str,
    pub password: &'a str,
    pub host: &'a str,
    pub port: u16,
    pub database_name: &'a str,
}

impl PostgresDatabase {
    fn get_pool_config(opts: &PostgresConfig) -> deadpool_postgres::Config {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.user = Some(opts.user.to_string());
        cfg.password = Some(opts.password.to_string());
        cfg.host = Some(opts.host.to_string());
        cfg.port = Some(opts.port);
        cfg.dbname = Some(opts.database_name.to_string());
        cfg
    }

    /// This function connect with database and runs migrations on it, after doing so it's fully
    /// ready for operations
    pub async fn new(opts: &PostgresConfig<'_>) -> Result<Self, Error> {
        use std::ops::DerefMut;

        let pool_config = Self::get_pool_config(&opts);
        let pool = pool_config
            .create_pool(NoTls)
            .expect("invalid pool configuration");
        let mut obj = pool.get().await?;
        let client = obj.deref_mut().deref_mut();
        migrations::runner().run_async(client).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl Database for PostgresDatabase {
    async fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error> {
        const QUERY: &str = "SELECT * FROM devices WHERE id = $1";
        let connection = self.pool.get().await?;
        let row = match connection.query_opt(QUERY, &[&device_id]).await? {
            Some(row) => row,
            None => return Ok(None),
        };

        let device = Device {
            id: row.try_get("id")?,
            password_hash: row.try_get("password_hash")?,
            device_type: row.try_get("type")?,
            traits: row.try_get("traits")?,
            name: row.try_get("name")?,
            will_push_state: row.try_get("will_push_state")?,
            room: row.try_get("room")?,
            model: row.try_get("model")?,
            hw_version: Version::parse(row.try_get("hw_version")?).map_err(|err| {
                PostgresError::InvalidColumn {
                    column: "hw_version",
                    error: err.into(),
                }
            })?,
            sw_version: Version::parse(row.try_get("sw_version")?).map_err(|err| {
                PostgresError::InvalidColumn {
                    column: "sw_version",
                    error: err.into(),
                }
            })?,
            attributes: row.try_get("attributes")?,
        };

        Ok(Some(device))
    }

    async fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error> {
        const QUERY: &str = "SELECT * FROM users WHERE id = $1";
        let connection = self.pool.get().await?;
        let row = match connection.query_opt(QUERY, &[&user_id]).await? {
            Some(row) => row,
            None => return Ok(None),
        };
        let user = User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
        };

        Ok(Some(user))
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        const QUERY: &str = "SELECT * FROM users WHERE email = $1";
        let connection = self.pool.get().await?;
        let row = match connection.query_opt(QUERY, &[&email.to_string()]).await? {
            Some(row) => row,
            None => return Ok(None),
        };
        let user = User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
        };

        Ok(Some(user))
    }

    async fn add_user(&self, user: &User) -> Result<(), Error> {
        const QUERY: &str = "
            INSERT INTO users(id, username, email, password_hash) 
            VALUES ($1, $2, $3, $4)
        ";
        let connection = self.pool.get().await?;
        let n = connection
            .execute(
                QUERY,
                &[&user.id, &user.username, &user.email, &user.password_hash],
            )
            .await?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    async fn delete_user(&self, user_id: &UserID) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM users WHERE id = $1";
        let connection = self.pool.get().await?;
        let n = connection.execute(QUERY, &[&user_id]).await?;
        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }
}
