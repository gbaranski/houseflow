use deadpool_postgres::Pool;
use std::ops::DerefMut;
use thiserror::Error;
use tokio_postgres::NoTls;

use refinery::embed_migrations;
embed_migrations!("migrations");

mod device;
mod user;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error when sending query: `{0}`")]
    QueryError(#[from] tokio_postgres::Error),

    #[error("Error occured with Postgres Pool: `{0}`")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("Error when running migrations: `{0}`")]
    MigrationError(#[from] refinery::Error),

    #[error("Column `{column}` is invalid: `{error}`")]
    InvalidColumn {
        column: &'static str,
        error: Box<dyn std::error::Error>,
    },

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
}
