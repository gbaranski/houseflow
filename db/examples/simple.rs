use houseflow_db::Database;
use std::convert::TryFrom;

const DATABASE_USER: &str = "postgres";
const DATABASE_PASSWORD: &str = "haslo123";
const DATABASE_HOST: &str = "localhost";
const DATABASE_PORT: u16 = 5432;
const DATABASE_NAME: &str = "houseflow";

const SALT: &[u8] = b"SomeSalt";
const USER_ID: &str = "19632fc07d08424ab80adfd907c3932c";
const USER_FIRST_NAME: &str = "John";
const USER_LAST_NAME: &str = "Smith";
const USER_EMAIL: &str = "root@gbaranski.com";
const USER_PASSWORD: &str = "haslo123";

#[tokio::main]
async fn main() {
    let password_hash =
        argon2::hash_encoded(USER_PASSWORD.as_bytes(), SALT, &argon2::Config::default()).unwrap();
    let user = houseflow_types::User {
        id: houseflow_types::UserID::try_from(String::from(USER_ID)).unwrap(),
        first_name: USER_FIRST_NAME.into(),
        last_name: USER_LAST_NAME.into(),
        email: USER_EMAIL.into(),
        password_hash,
    };
    let database_options = houseflow_db::Options {
        user: DATABASE_USER.into(),
        password: DATABASE_PASSWORD.into(),
        host: DATABASE_HOST.into(),
        port: DATABASE_PORT,
        database_name: DATABASE_NAME.into(),
    };
    let database = Database::new(database_options)
        .await
        .expect("failed creating database");
    database.add_user(&user).await.expect("add user failed");
    let db_user = database
        .get_user(&user.id)
        .await
        .expect("get user failed")
        .expect("user not found");
    assert_eq!(user, db_user);
    database
        .delete_user(&user.id)
        .await
        .expect("delete user failed");
}

