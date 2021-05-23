use deadpool_postgres::Pool;
use houseflow_types::{User, UserID};
use std::convert::TryFrom;
use std::ops::DerefMut;
use thiserror::Error;
use tokio_postgres::NoTls;

use refinery::embed_migrations;
embed_migrations!("migrations");

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error when sending query: {0}")]
    QueryError(#[from] tokio_postgres::Error),

    #[error("Error occured with Postgres Pool: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("Error when running migrations: {0}")]
    MigrationError(#[from] refinery::Error),

    #[error("Query did not modify anything")]
    NotModified,
}

pub struct Options {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(Clone)]
pub struct Database {
    pool: Pool<NoTls>,
}

impl Database {
    fn get_pool_config(opts: &Options) -> deadpool_postgres::Config {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.user = Some(opts.user.clone());
        cfg.password = Some(opts.password.clone());
        cfg.host = Some(opts.host.clone());
        cfg.port = Some(opts.port);
        cfg.dbname = Some(opts.database_name.clone());
        cfg
    }

    /// This function connect with database and runs migrations on it, after doing so it's fully
    /// ready for operations
    pub async fn new(opts: Options) -> Result<Self, Error> {
        let pool_config = Self::get_pool_config(&opts);
        let pool = pool_config
            .create_pool(NoTls)
            .expect("invalid pool configuration");
        let mut obj = pool.get().await?;
        let client = obj.deref_mut().deref_mut();
        migrations::runner().run_async(client).await?;
        Ok(Self { pool })
    }

    pub async fn add_user(&self, user: &User) -> Result<(), Error> {
        const QUERY: &str = "
            INSERT INTO users(id, first_name, last_name, email, password_hash) 
            VALUES ($1, $2, $3, $4, $5)
        ";
        let connection = self.pool.get().await?;
        let n = connection
            .execute(
                QUERY,
                &[
                    &user.id.to_string(),
                    &user.first_name,
                    &user.last_name,
                    &user.email,
                    &user.password_hash,
                ],
            )
            .await?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    pub async fn delete_user(&self, user_id: &UserID) -> Result<(), Error> {
        const QUERY: &str = "DELETE FROM users WHERE id = $1";
        let connection = self.pool.get().await?;
        let n = connection.execute(QUERY, &[&user_id.to_string()]).await?;
        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    pub async fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error> {
        const QUERY: &str = "SELECT * FROM users WHERE id = $1";
        let connection = self.pool.get().await?;
        let row = match connection.query_opt(QUERY, &[&user_id.to_string()]).await? {
            Some(row) => row,
            None => return Ok(None),
        };
        let user = User {
            id: UserID::try_from(row.try_get::<_, &str>("id")?).unwrap(),
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
        };

        Ok(Some(user))
    }

    //     pub async fn get_device_by_id(&self, device_id: DeviceID) -> Result<Option<Device>, Error> {
    //         const QUERY: &str = "SELECT * FROM devices WHERE id = $1";
    //
    //         let device_id: &str = &device_id.to_string();
    //         let device = sqlx::query_as::<sqlx::Postgres, Device>(QUERY)
    //             .bind(device_id)
    //             .fetch_optional(&self.pool)
    //             .await?;
    //         Ok(device)
    //     }
}
