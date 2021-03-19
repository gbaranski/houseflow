use serde::{Serialize, Deserialize};

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
CREATE TABLE IF NOT EXISTS user_devices (
    id UUID NOT NULL,
    user_id UUID REFERENCES users (id) ON DELETE CASCADE,
    device_id UUID REFERENCES devices (id) ON DELETE CASCADE,
    read BOOL NOT NULL,
    write BOOL NOT NULL,
    execute BOOL NOT NULL,

    PRIMARY KEY(id)
);
"#;


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub async fn by_id(db: &crate::Database, id: String) -> Result<Option<User>, crate::Error> {
        const SQL_QUERY: &str = 
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
        let client = db.pool.get().await?;
        let row = client
            .query_one(SQL_QUERY, &[&id])
            .await?;

        if row.is_empty() {
            return Ok(None)
        }

        Ok(Some(User{
            id,
            first_name: row.try_get(0)?,
            last_name: row.try_get(1)?,
            email: row.try_get(2)?,
            password_hash: row.try_get(3)?,
        }))
    }
}
