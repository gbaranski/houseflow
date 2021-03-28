use serde::{ser, Serialize, Deserialize};
use uuid::Uuid;

pub const USER_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,

    PRIMARY KEY(id)
);
"#;


pub const USER_DEVICES_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS user_permissions (
    user_id UUID REFERENCES users (id) ON DELETE CASCADE,
    device_id UUID REFERENCES devices (id) ON DELETE CASCADE,
    read BOOL NOT NULL,
    write BOOL NOT NULL,
    execute BOOL NOT NULL,

    PRIMARY KEY(user_id, device_id)
);
"#;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
}

pub struct DevicePermission {
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}


impl crate::Database {
    pub async fn get_device_permission(
        &self, 
        user_id: Uuid, 
        device_id: Uuid
    ) -> Result<Option<DevicePermission>, crate::Error> {
        const SQL_QUERY: &'static str = 
            r#"
            SELECT
                read,
                write,
                execute
            FROM 
                user_permissions
            WHERE
                user_id=$1
            AND
                device_id=$2
            "#;
        let client = self.pool.get().await?;
        let row = client
            .query_one(SQL_QUERY, &[&user_id, &device_id])
            .await?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DevicePermission {
                user_id,
                device_id,
                read: row.try_get(0)?,
                write: row.try_get(1)?,
                execute: row.try_get(2)?,
            }))
        }

    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, crate::Error> {
        const SQL_QUERY: &'static str = 
        r#"
        "SELECT 
            first_name,
            last_name,
            email,
            password_hash,
        FROM 
            users 
        WHERE 
            id=$1"
        "#;
        let client = self.pool.get().await?;
        let row = client
            .query_one(SQL_QUERY, &[&id])
            .await?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(User{
                id,
                first_name: row.try_get(0)?,
                last_name: row.try_get(1)?,
                email: row.try_get(2)?,
                password_hash: row.try_get(3)?,
            }))
        }

    }
}
