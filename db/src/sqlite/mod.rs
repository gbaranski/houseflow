use houseflow_types::{
    Device, DeviceID, DeviceTrait, Room, RoomID, Structure, StructureID, User, UserID,
};
use r2d2_sqlite::SqliteConnectionManager;
use semver::Version;
use std::{path::Path, str::FromStr};

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
    fn add_structure(&self, structure: &Structure) -> Result<(), Error> {
        const SQL: &str = "INSERT INTO structures(id,name) VALUES(?, ?)";
        let connection = self.pool.get()?;
        let n = connection.execute(SQL, params![&structure.id, &structure.name])?;
        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    fn add_room(&self, room: &Room) -> Result<(), Error> {
        const SQL: &str = "INSERT INTO rooms(id, structure_id, name) VALUES(?, ?, ?)";
        let connection = self.pool.get()?;
        let n = connection.execute(SQL, params![&room.id, &room.structure_id, &room.name])?;
        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    fn add_device(&self, device: &Device) -> Result<(), Error> {
        const INSERT_DEVICE_SQL: &str = "INSERT INTO 
            devices(id, room_id, password_hash, type, name, will_push_state, model, hw_version, sw_version, attributes) 
            VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        const INSERT_TRAIT_SQL: &str = "INSERT INTO device_traits(device_id, trait_name) 
            VALUES(?, ?)";

        let mut connection = self.pool.get()?;
        let tx = connection.transaction()?;
        let n = tx.execute(
            INSERT_DEVICE_SQL,
            params![
                device.id,
                device.room_id,
                device.password_hash,
                device.device_type,
                device.name,
                device.will_push_state,
                device.model,
                device.hw_version.to_string(),
                device.sw_version.to_string(),
                serde_json::to_string(&device.attributes)?
            ],
        )?;
        if n == 0 {
            return Err(Error::NotModified);
        }
        for device_trait in &device.traits {
            let n = tx.execute(INSERT_TRAIT_SQL, params!(&device.id, &device_trait))?;
            if n == 0 {
                return Err(Error::NotModified);
            }
        }
        tx.commit()?;
        Ok(())
    }

    fn add_user(&self, user: &User) -> Result<(), Error> {
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

    fn add_user_structure(
        &self,
        user_structure: &houseflow_types::UserStructure,
    ) -> Result<(), Error> {
        const SQL: &str =
            "INSERT INTO user_structures(structure_id, user_id, is_manager) VALUES(?, ?, ?)";
        let connection = self.pool.get()?;
        let n = connection.execute(
            SQL,
            params![
                &user_structure.structure_id,
                &user_structure.user_id,
                &user_structure.is_manager
            ],
        )?;
        match n {
            0 => Err(Error::NotModified),
            1 => Ok(()),
            _ => unreachable!(),
        }
    }

    fn get_structure(&self, structure_id: &StructureID) -> Result<Option<Structure>, Error> {
        const SQL: &str = "SELECT * FROM structures WHERE id = ?";
        let connection = self.pool.get()?;
        let structure = connection
            .query_row(SQL, params![structure_id], |row| {
                Ok(Structure {
                    id: row.get("id")?,
                    name: row.get("name")?,
                })
            })
            .optional()?;

        Ok(structure)
    }

    fn get_room(&self, room_id: &RoomID) -> Result<Option<Room>, Error> {
        const SQL: &str = "SELECT * FROM rooms WHERE id = ?";
        let connection = self.pool.get()?;
        let room = connection
            .query_row(SQL, params![room_id], |row| {
                Ok(Room {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    structure_id: row.get("structure_id")?,
                })
            })
            .optional()?;

        Ok(room)
    }

    fn get_device(&self, device_id: &DeviceID) -> Result<Option<Device>, Error> {
        const SELECT_DEVICES_SQL: &str = "SELECT * FROM devices WHERE id = ?";
        const SELECT_TRAITS_SQL: &str = "SELECT trait_name FROM device_traits WHERE device_id = ?";

        use fallible_iterator::FallibleIterator;

        let connection = self.pool.get()?;
        let mut select_traits_sql = connection.prepare(SELECT_TRAITS_SQL)?;
        let traits: Vec<DeviceTrait> = select_traits_sql
            .query(params![device_id])?
            .map(|row| {
                DeviceTrait::from_str(row.get::<_, String>("trait_name")?.as_str())
                    .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)).into())
            })
            .collect()?;

        let device = connection
            .query_row(SELECT_DEVICES_SQL, params![device_id], |row| {
                Ok(Device {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    room_id: row.get("room_id")?,
                    password_hash: row.get("password_hash")?,
                    device_type: row.get("type")?,
                    traits,
                    will_push_state: row.get("will_push_state")?,
                    model: row.get("model")?,
                    hw_version: Version::parse(row.get::<_, String>("hw_version")?.as_str())
                        .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)))?,
                    sw_version: Version::parse(row.get::<_, String>("sw_version")?.as_str())
                        .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)))?,
                    attributes: serde_json::from_str(row.get::<_, String>("attributes")?.as_str())
                        .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)))?,
                })
            })
            .optional()?;

        Ok(device)
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

    fn get_user_devices(&self, user_id: &UserID) -> Result<Vec<Device>, Error> {
        const SELECT_TRAITS_SQL: &str = "SELECT trait_name FROM device_traits WHERE device_id = ?";
        const SELECT_USER_DEVICES_SQL: &str = "
            SELECT * 
            FROM devices 
            WHERE room_id = (
                SELECT id
                FROM rooms
                WHERE structure_id = (
                    SELECT structure_id
                    FROM user_structures
                    WHERE user_id = ?
                )
            )";

        use fallible_iterator::FallibleIterator;

        let connection = self.pool.get()?;
        let mut select_traits_sql = connection.prepare(SELECT_TRAITS_SQL)?;
        let mut statement = connection.prepare(SELECT_USER_DEVICES_SQL)?;
        let devices = statement
            .query(params![user_id])?
            .map(|row| {
                let device_id = row.get("id")?;
                let traits: Vec<DeviceTrait> = select_traits_sql
                    .query(params![device_id])?
                    .map(|row| {
                        DeviceTrait::from_str(row.get::<_, String>("trait_name")?.as_str()).map_err(
                            |err| rusqlite::types::FromSqlError::Other(Box::new(err)).into(),
                        )
                    })
                    .collect()?;
                Ok(Device {
                    id: device_id,
                    room_id: row.get("room_id")?,
                    password_hash: row.get("password_hash")?,
                    device_type: row.get("type")?,
                    traits,
                    name: row.get("name")?,
                    will_push_state: row.get("will_push_state")?,
                    model: row.get("model")?,
                    hw_version: Version::parse(row.get::<_, String>("hw_version")?.as_str())
                        .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)))?,
                    sw_version: Version::parse(row.get::<_, String>("sw_version")?.as_str())
                        .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)))?,
                    attributes: serde_json::from_str(row.get::<_, String>("attributes")?.as_str())
                        .map_err(|err| rusqlite::types::FromSqlError::Other(Box::new(err)))?,
                })
            })
            .collect()?;

        Ok(devices)
    }

    fn check_user_device_access(
        &self,
        user_id: &UserID,
        device_id: &DeviceID,
    ) -> Result<bool, Error> {
        const SQL: &str = "
            SELECT 1
            FROM devices 
            WHERE id = ? 
            AND room_id = (
                SELECT id
                FROM rooms
                WHERE structure_id = (
                    SELECT structure_id
                    FROM user_structures
                    WHERE user_id = ?
                )
            )
            ";
        let connection = self.pool.get()?;
        let result = connection
            .query_row(SQL, params![device_id, user_id], |_| Ok(()))
            .optional()?;

        Ok(result.is_some())
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
    use houseflow_types::{
        Device, DeviceTrait, DeviceType, Room, RoomID, Structure, StructureID, User, UserID,
        UserStructure,
    };
    use rand::random;
    use semver::Version;

    fn get_database() -> SqliteDatabase {
        SqliteDatabase::new_in_memory().unwrap()
    }

    mod structure {
        use super::*;

        pub fn gen() -> Structure {
            Structure {
                id: random(),
                name: "SomeStructure".to_string(),
            }
        }

        #[test]
        fn add() {
            let db = get_database();
            let structure = gen();
            db.add_structure(&structure).unwrap();
            assert_eq!(db.get_structure(&structure.id).unwrap().unwrap(), structure)
        }

        #[test]
        fn add_duplicate() {
            let db = get_database();
            let structure = gen();
            db.add_structure(&structure).unwrap();
            db.add_structure(&structure).unwrap_err();
        }
    }

    mod room {
        use super::*;

        pub fn gen(structure_id: StructureID) -> Room {
            Room {
                id: random(),
                name: "SomeRoom".to_string(),
                structure_id: structure_id.clone(),
            }
        }

        #[test]
        fn add_get() {
            let db = get_database();
            let structure = super::structure::gen();
            let room = gen(structure.id.clone());
            db.add_structure(&structure).unwrap();
            db.add_room(&room).unwrap();
            assert_eq!(db.get_room(&room.id).unwrap().unwrap(), room)
        }

        #[test]
        fn add_duplicate() {
            let db = get_database();
            let structure = super::structure::gen();
            let room = gen(structure.id.clone());
            db.add_structure(&structure).unwrap();
            db.add_room(&room).unwrap();
            db.add_room(&room).unwrap_err();
        }

        #[test]
        fn add_no_structure() {
            let db = get_database();
            let structure = super::structure::gen();
            let room = gen(structure.id.clone());
            db.add_room(&room).unwrap_err();
        }
    }

    mod device {
        use super::*;

        pub fn gen(room_id: RoomID) -> Device {
            Device {
                id: random(),
                name: "SomeRoom".to_string(),
                room_id: room_id.clone(),
                password_hash: "SomePasswordHash".to_string(),
                device_type: DeviceType::Garage,
                traits: vec![DeviceTrait::OnOff, DeviceTrait::OpenClose],
                will_push_state: true,
                model: "testing-garage".to_string(),
                hw_version: Version::new(1, 0, 0),
                sw_version: Version::new(1, 0, 0),
                attributes: Default::default(),
            }
        }

        #[test]
        fn add_get() {
            let db = get_database();
            let structure = super::structure::gen();
            let room = super::room::gen(structure.id.clone());
            let device = gen(room.id.clone());
            db.add_structure(&structure).unwrap();
            db.add_room(&room).unwrap();
            db.add_device(&device).unwrap();
            assert_eq!(db.get_device(&device.id).unwrap().unwrap(), device)
        }

        #[test]
        fn add_duplicate() {
            let db = get_database();
            let structure = super::structure::gen();
            let room = super::room::gen(structure.id.clone());
            let device = gen(room.id.clone());
            db.add_structure(&structure).unwrap();
            db.add_room(&room).unwrap();
            db.add_device(&device).unwrap();
            db.add_device(&device).unwrap_err();
        }

        #[test]
        fn add_no_room() {
            let db = get_database();
            let device = gen(random());
            db.add_device(&device).unwrap_err();
        }
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
        fn add_get() {
            let db = get_database();
            let user = gen();
            db.add_user(&user).unwrap();
            assert_eq!(db.get_user(&user.id).unwrap().unwrap(), user);
            assert_eq!(db.get_user_by_email(&user.email).unwrap().unwrap(), user);
            assert_eq!(db.check_user_admin(&user.id).unwrap(), false);
            db.add_admin(&user.id).unwrap();
            assert_eq!(db.check_user_admin(&user.id).unwrap(), true);
        }

        #[test]
        fn add_duplicate() {
            let db = get_database();
            let user = gen();
            db.add_user(&user).unwrap();
            db.add_user(&user).unwrap_err();
        }
    }

    mod user_structure {
        use super::*;

        pub fn gen(user_id: UserID, structure_id: StructureID, is_manager: bool) -> UserStructure {
            UserStructure {
                structure_id,
                user_id,
                is_manager,
            }
        }

        #[test]
        fn add_get() {
            let db = get_database();
            let structure_allow = super::structure::gen();
            let structure_deny = super::structure::gen();
            let room_allow = super::room::gen(structure_allow.id.clone());
            let room_deny = super::room::gen(structure_deny.id.clone());
            let device_allow = super::device::gen(room_allow.id.clone());
            let device_deny = super::device::gen(room_deny.id.clone());
            let user = super::user::gen();
            let user_structure = gen(user.id.clone(), structure_allow.id.clone(), false);
            db.add_user(&user).unwrap();
            db.add_structure(&structure_allow).unwrap();
            db.add_structure(&structure_deny).unwrap();
            db.add_room(&room_allow).unwrap();
            db.add_room(&room_deny).unwrap();
            db.add_device(&device_allow).unwrap();
            db.add_device(&device_deny).unwrap();

            db.add_user_structure(&user_structure).unwrap();

            assert_eq!(
                db.check_user_device_access(&user.id, &device_allow.id)
                    .unwrap(),
                true
            );
            assert_eq!(
                db.check_user_device_access(&user.id, &device_deny.id)
                    .unwrap(),
                false
            );
            assert_eq!(db.get_user_devices(&user.id).unwrap(), vec![device_allow]);
        }

        #[test]
        fn add_duplicate() {
            let db = get_database();
            let structure = super::structure::gen();
            let user = super::user::gen();
            let user_structure = gen(user.id.clone(), structure.id.clone(), false);
            db.add_user(&user).unwrap();
            db.add_structure(&structure).unwrap();
            db.add_user_structure(&user_structure).unwrap();
            db.add_user_structure(&user_structure).unwrap_err();
        }
    }
}
