use houseflow_types::{User, UserID};
use crate::{Database, Error};

impl Database {
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
                    &user.id,
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
        let n = connection.execute(QUERY, &[&user_id]).await?;
        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    pub async fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error> {
        const QUERY: &str = "SELECT * FROM users WHERE id = $1";
        let connection = self.pool.get().await?;
        let row = match connection.query_opt(QUERY, &[&user_id]).await? {
            Some(row) => row,
            None => return Ok(None),
        };
        let user = User {
            id: row.try_get("id")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            email: row.try_get("email")?,
            password_hash: row.try_get("password_hash")?,
        };

        Ok(Some(user))
    }

}
