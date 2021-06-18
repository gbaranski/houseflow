use houseflow_db::{postgres, Database};
use std::str::FromStr;
use std::time::Instant;

const DATABASE_USER: &str = "postgres";
const DATABASE_PASSWORD: &str = "haslo123";
const DATABASE_HOST: &str = "localhost";
const DATABASE_PORT: u16 = 5432;
const DATABASE_NAME: &str = "houseflow";

const SALT: &[u8] = b"SomeSalt";
const USER_ID: &str = "19632fc07d08424ab80adfd907c3932c";
const USERNAME: &str = "gbaranski";
const USER_EMAIL: &str = "username@example.com";
const USER_PASSWORD: &str = "somepassword";

#[tokio::main]
async fn main() {
    let password_hash =
        argon2::hash_encoded(USER_PASSWORD.as_bytes(), SALT, &argon2::Config::default()).unwrap();
    let user = types::User {
        id: types::UserID::from_str(USER_ID).unwrap(),
        username: USERNAME.into(),
        email: USER_EMAIL.into(),
        password_hash,
    };
    let database_config = postgres::Config {
        user: DATABASE_USER.into(),
        password: DATABASE_PASSWORD.into(),
        host: DATABASE_HOST.into(),
        port: DATABASE_PORT,
        database_name: DATABASE_NAME.into(),
    };
    let database = postgres::Database::new(&database_config)
        .await
        .expect("failed creating database");
    let start = Instant::now();
    database.add_user(&user).await.expect("add user failed");
    println!("Added user");
    let db_user = database
        .get_user(&user.id)
        .await
        .expect("get user failed")
        .expect("user not found");
    println!("Retreived user");
    assert_eq!(user, db_user);
    database
        .delete_user(&user.id)
        .await
        .expect("delete user failed");
    println!("Deleted user");
    println!("All actions took: {:#?}", start.elapsed())
}
