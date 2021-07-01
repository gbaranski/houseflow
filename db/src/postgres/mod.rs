use crate::Error;

use async_trait::async_trait;
use deadpool_postgres::Pool;
use houseflow_config::postgres::Config;
use houseflow_types::{Device, DeviceID, User, UserID, UserStructure};
use semver::Version;
use tokio_postgres::NoTls;

use refinery::embed_migrations;
embed_migrations!("migrations");

#[derive(Debug, thiserror::Error)]
pub enum InternalError {
    #[error("Error when sending query: `{0}`")]
    QueryError(#[from] tokio_postgres::Error),

    #[error("pool error: {0}")]
    PoolError(#[from] deadpool_postgres::PoolError),

    #[error("Column `{column}` is invalid: `{error}`")]
    InvalidColumn {
        column: &'static str,
        error: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Error when running migrations: `{0}`")]
    MigrationError(#[from] refinery::Error),
}

use crate::DatabaseInternalError;

impl DatabaseInternalError for InternalError {}
impl DatabaseInternalError for deadpool_postgres::PoolError {}
impl DatabaseInternalError for tokio_postgres::Error {}
impl DatabaseInternalError for refinery::Error {}

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    fn get_pool_config(cfg: &Config) -> deadpool_postgres::Config {
        let mut dpcfg = deadpool_postgres::Config::new();
        dpcfg.user = Some(cfg.user.to_string());
        dpcfg.password = Some(cfg.password.to_string());
        dpcfg.host = Some(cfg.address.ip().to_string());
        dpcfg.port = Some(cfg.address.port());
        dpcfg.dbname = Some(cfg.database_name.to_string());
        dpcfg
    }

    /// This function connect with database and runs migrations on it, after doing so it's fully
    /// ready for operations
    pub async fn new(opts: &Config) -> Result<Self, Error> {
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
impl crate::Database for Database {
    async fn add_structure(&self, structure: &houseflow_types::Structure) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let insert_statement = connection
            .prepare(
                r#"
            INSERT INTO structures (id, name) 
            VALUES ($1, $2)
            "#,
            )
            .await?;

        let n = connection
            .execute(&insert_statement, &[&structure.id, &structure.name])
            .await?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    async fn add_room(&self, room: &houseflow_types::Room) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let insert_statement = connection
            .prepare(
                r#"
            INSERT INTO rooms (id, structure_id, name) 
            VALUES ($1, $2, $3)
            "#,
            )
            .await?;

        let n = connection
            .execute(
                &insert_statement,
                &[&room.id, &room.structure_id, &room.name],
            )
            .await?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    async fn add_device(&self, device: &Device) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let insert_statement = connection.prepare(
            r#"
            INSERT INTO devices(
                id, room_id, password_hash, type, traits, name, will_push_state, model, hw_version, sw_version, attributes
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        ).await?;

        let n = connection
            .execute(
                &insert_statement,
                &[
                    &device.id,
                    &device.room_id,
                    &device.password_hash,
                    &device.device_type.to_string(),
                    &device
                        .traits
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>(),
                    &device.name,
                    &device.will_push_state,
                    &device.model,
                    &device.hw_version.to_string(),
                    &device.sw_version.to_string(),
                    &device.attributes,
                ],
            )
            .await?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    async fn add_user_structure(&self, user_structure: &UserStructure) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let insert_statement = connection
            .prepare(
                r#"
            INSERT INTO user_structures (structure_id, user_id, is_manager) 
            VALUES ($1, $2, $3)
            "#,
            )
            .await?;

        let n = connection
            .execute(
                &insert_statement,
                &[
                    &user_structure.structure_id,
                    &user_structure.user_id,
                    &user_structure.is_manager,
                ],
            )
            .await?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    async fn add_user(&self, user: &User) -> Result<(), Error> {
        let connection = self.pool.get().await?;
        let check_exists_statement = connection.prepare(
            r#"
            SELECT 1
            FROM users 
            WHERE email = $1
            OR username = $2
            "#,
        );

        let insert_statement = connection.prepare(
            r#"
            INSERT INTO users(id, username, email, password_hash) 
            VALUES ($1, $2, $3, $4)
            "#,
        );

        let (check_exists_statement, insert_statement) =
            tokio::join!(check_exists_statement, insert_statement);

        let (check_exists_statement, insert_statement) =
            (check_exists_statement?, insert_statement?);

        let exists = connection
            .query_opt(&check_exists_statement, &[&user.email, &user.username])
            .await?
            .is_some();

        if exists {
            return Err(Error::AlreadyExists);
        }

        let n = connection
            .execute(
                &insert_statement,
                &[&user.id, &user.username, &user.email, &user.password_hash],
            )
            .await?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    async fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error> {
        const QUERY: &str = "
            SELECT * 
            FROM devices 
            WHERE id = $1";
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
            room_id: row.try_get("room_id")?,
            model: row.try_get("model")?,
            hw_version: Version::parse(row.try_get("hw_version")?).map_err(|err| {
                InternalError::InvalidColumn {
                    column: "hw_version",
                    error: Box::new(err),
                }
            })?,
            sw_version: Version::parse(row.try_get("sw_version")?).map_err(|err| {
                InternalError::InvalidColumn {
                    column: "sw_version",
                    error: Box::new(err),
                }
            })?,
            attributes: row.try_get("attributes")?,
        };

        Ok(Some(device))
    }

    async fn get_user_devices(&self, user_id: &UserID) -> Result<Vec<Device>, Error> {
        let connection = self.pool.get().await?;
        let query_statement = connection
            .prepare(
                r#"
            SELECT *
            FROM devices
            WHERE room_id = (
                SELECT id 
                FROM rooms 
                WHERE structure_id = (
                    SELECT structure_id
                    FROM user_structures
                    WHERE user_id = $1
                )
            )
            "#,
            )
            .await?;
        let row = connection.query(&query_statement, &[&user_id]).await?;
        let devices = row.iter().map(|row| {
            Ok::<Device, Error>(Device {
                id: row.try_get("id")?,
                room_id: row.try_get("room_id")?,
                password_hash: row.try_get("password_hash")?,
                device_type: row.try_get("type")?,
                traits: row.try_get("traits")?,
                name: row.try_get("name")?,
                will_push_state: row.try_get("will_push_state")?,
                model: row.try_get("model")?,
                hw_version: Version::parse(row.try_get("hw_version")?).map_err(|err| {
                    InternalError::InvalidColumn {
                        column: "hw_version",
                        error: Box::new(err),
                    }
                })?,
                sw_version: Version::parse(row.try_get("sw_version")?).map_err(|err| {
                    InternalError::InvalidColumn {
                        column: "sw_version",
                        error: Box::new(err),
                    }
                })?,
                attributes: row.try_get("attributes")?,
            })
        });
        let devices: Result<Vec<Device>, Error> = devices.collect();
        devices
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

    async fn check_user_device_access(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
    ) -> Result<bool, Error> {
        let connection = self.pool.get().await?;
        let query_statement = connection
            .prepare(
                r#"
            SELECT 1
            FROM devices
            WHERE id = $1
            AND room_id = ( 
                SELECT id 
                FROM rooms 
                WHERE structure_id = (
                    SELECT structure_id
                    FROM user_structures
                    WHERE user_id = $2
                )
            )
            "#,
            )
            .await?;
        let result = connection
            .query_opt(&query_statement, &[&device_id, &user_id])
            .await?;

        Ok(result.is_some())
    }

    async fn check_user_device_manager_access(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
    ) -> Result<bool, Error> {
        let connection = self.pool.get().await?;
        let query_statement = connection
            .prepare(
                r#"
            SELECT 1
            FROM devices
            WHERE id = $1
            AND room_id = ( 
                SELECT id 
                FROM rooms 
                WHERE structure_id = (
                    SELECT structure_id
                    FROM user_structures
                    WHERE user_id = $2
                    AND is_manager = true
                )
            )
            "#,
            )
            .await?;
        let result = connection
            .query_opt(&query_statement, &[&device_id, &user_id])
            .await?;

        Ok(result.is_some())
    }

    async fn check_user_admin(&self, user_id: &UserID) -> Result<bool, Error> {
        let connection = self.pool.get().await?;
        let query_statement = connection
            .prepare(
                r#"
            SELECT 1
            FROM admins
            WHERE user_id = $1
            "#,
            )
            .await?;

        let result = connection.query_opt(&query_statement, &[&user_id]).await?;

        Ok(result.is_some())
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
