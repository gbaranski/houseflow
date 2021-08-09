use houseflow_types::{User, UserID};
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;

#[derive(Clone)]
pub struct Database {
    pool: r2d2::Pool<SqliteConnectionManager>,
}

use crate::Error;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

impl Database {
    fn init(manager: SqliteConnectionManager) -> Result<Self, Error> {
        use std::ops::DerefMut;

        let pool = r2d2::Pool::new(manager)?;
        let mut connection = pool.get()?;
        connection.execute("PRAGMA foreign_keys = ON", params!())?;
        embedded::migrations::runner().run(connection.deref_mut())?;
        Ok(Self { pool })
    }

    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let manager = SqliteConnectionManager::file(path);

        Self::init(manager)
    }

    pub fn new_in_memory() -> Result<Self, Error> {
        let manager = SqliteConnectionManager::memory();

        Self::init(manager)
    }
}

use rusqlite::{params, OptionalExtension};

impl crate::Database for Database {
    fn add_user(&self, user: &User) -> Result<(), Error> {
        if self.get_user_by_email(&user.email)?.is_some() {
            return Err(Error::AlreadyExists);
        };
        const SQL: &str =
            "INSERT INTO users(id, username, email, password_hash) VALUES(?, ?, ?, ?)";
        let connection = self.pool.get()?;
        let n = connection.execute(
            SQL,
            params![&user.id, &user.username, &user.email, &user.password_hash],
        )?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    fn add_admin(&self, user_id: &UserID) -> Result<(), Error> {
        const SQL: &str = "INSERT INTO admins(user_id) VALUES(?)";
        let connection = self.pool.get()?;
        let n = connection.execute(SQL, params![user_id])?;

        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    fn delete_user(&self, user_id: &UserID) -> Result<bool, Error> {
        const SQL: &str = "DELETE FROM users WHERE id = ?";
        let connection = self.pool.get()?;
        let n = connection.execute(SQL, params![user_id])?;
        Ok(n > 0)
    }

    fn delete_admin(&self, user_id: &UserID) -> Result<bool, Error> {
        const SQL: &str = "DELETE FROM admins WHERE user_id = ?";
        let connection = self.pool.get()?;
        let n = connection.execute(SQL, params![user_id])?;
        Ok(n > 0)
    }

    fn get_user(&self, user_id: &UserID) -> Result<Option<User>, Error> {
        const SQL: &str = "SELECT * FROM users WHERE id = ?";
        let connection = self.pool.get()?;
        let user = connection
            .query_row(SQL, params![user_id], |row| {
                Ok(User {
                    id: row.get("id")?,
                    username: row.get("username")?,
                    email: row.get("email")?,
                    password_hash: row.get("password_hash")?,
                })
            })
            .optional()?;

        Ok(user)
    }

    fn get_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        const SQL: &str = "SELECT * FROM users WHERE email = ?";
        let connection = self.pool.get()?;
        let user = connection
            .query_row(SQL, params![email], |row| {
                Ok(User {
                    id: row.get("id")?,
                    username: row.get("username")?,
                    email: row.get("email")?,
                    password_hash: row.get("password_hash")?,
                })
            })
            .optional()?;

        Ok(user)
    }

    fn check_user_admin(&self, user_id: &UserID) -> Result<bool, Error> {
        const SQL: &str = "
            SELECT 1
            FROM admins 
            WHERE user_id = ?
            ";

        let connection = self.pool.get()?;
        let result = connection
            .query_row(SQL, params![user_id], |_| Ok(()))
            .optional()?;

        Ok(result.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::Database as SqliteDatabase;
    use crate::Database;
    use houseflow_types::User;
    use rand::random;

    fn get_database() -> SqliteDatabase {
        SqliteDatabase::new_in_memory().unwrap()
    }

    mod user {
        use super::*;

        pub fn gen() -> User {
            User {
                id: random(),
                username: "gbaranski".to_string(),
                email: "root@gbaranski.com".to_string(),
                password_hash: "super-secret".to_string(),
            }
        }

        #[test]
        fn add_get_delete() {
            let db = get_database();
            let user = gen();
            db.add_user(&user).unwrap();
            assert_eq!(db.get_user(&user.id).unwrap().unwrap(), user);
            assert_eq!(db.get_user_by_email(&user.email).unwrap().unwrap(), user);
            assert_eq!(db.check_user_admin(&user.id).unwrap(), false);
            db.add_admin(&user.id).unwrap();
            assert_eq!(db.check_user_admin(&user.id).unwrap(), true);
            assert_eq!(db.delete_admin(&user.id).unwrap(), true);
            assert_eq!(db.check_user_admin(&user.id).unwrap(), false);
            assert_eq!(db.delete_admin(&user.id).unwrap(), false);
            assert_eq!(db.delete_user(&user.id).unwrap(), true);
            assert_eq!(db.get_user(&user.id).unwrap(), None);
            assert_eq!(db.check_user_admin(&user.id).unwrap(), false);
        }

        #[test]
        fn add_duplicate() {
            let db = get_database();
            let user = gen();
            db.add_user(&user).unwrap();
            db.add_user(&user).unwrap_err();
        }
    }
}
