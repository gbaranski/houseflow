mod admin;
mod auth;
mod fulfillment;
mod lighthouse;
mod oauth;
mod token_store;

pub use token_store::{sled::TokenStore as SledTokenStore, TokenStore};

use actix_web::web;
use houseflow_config::server::Config;
use houseflow_db::Database;

use {
    houseflow_types::DeviceID, lighthouse::Session, std::collections::HashMap, tokio::sync::Mutex,
};
pub type Sessions = Mutex<HashMap<DeviceID, actix::Addr<Session>>>;

pub(crate) fn get_password_salt() -> [u8; 16] {
    rand::random()
}

pub fn configure(
    cfg: &mut web::ServiceConfig,
    token_store: web::Data<dyn TokenStore>,
    database: web::Data<dyn Database>,
    config: web::Data<Config>,
    sessions: web::Data<Sessions>,
) {
    cfg.app_data(config)
        .app_data(token_store)
        .app_data(sessions)
        .app_data(database)
        .service(
            web::scope("/admin")
                .service(web::scope("/device").route("/add", web::put().to(admin::device::on_add)))
                .service(web::scope("/room").route("/add", web::put().to(admin::room::on_add)))
                .service(
                    web::scope("/structure").route("/add", web::put().to(admin::structure::on_add)),
                )
                .service(
                    web::scope("/user_structure")
                        .route("/add", web::put().to(admin::user_structure::on_add)),
                ),
        )
        .service(
            web::scope("/oauth")
                .route("/authorize", web::get().to(oauth::on_authorize))
                .route("/login", web::post().to(oauth::on_login))
                .service(
                    web::scope("/")
                        .app_data(oauth::on_exchange_refresh_token_form_config())
                        .route("/token", web::post().to(oauth::on_exchange_refresh_token)),
                ),
        )
        .service(
            web::scope("/auth")
                .route("/login", web::post().to(auth::on_login))
                .route("/logout", web::post().to(auth::on_logout))
                .route("/register", web::post().to(auth::on_register))
                .route("/whoami", web::get().to(auth::on_whoami)),
        )
        .service(
            web::scope("/fulfillment").service(
                web::scope("/internal")
                    .route(
                        "/execute",
                        web::post().to(fulfillment::internal::on_execute),
                    )
                    .route("/query", web::get().to(fulfillment::internal::on_query))
                    .route("/sync", web::get().to(fulfillment::internal::on_sync)),
            ),
        )
        .service(web::scope("/lighthouse").route("/ws", web::get().to(lighthouse::on_websocket)));
}

#[cfg(test)]
mod test_utils {
    use super::Config;
    use crate::{token_store, TokenStore};
    use houseflow_db::{sqlite::Database as SqliteDatabase, Database};
    use houseflow_types::{Device, DeviceType, Room, Structure, User, UserID};

    use actix_web::web::Data;
    use std::sync::Arc;

    pub const PASSWORD: &str = "SomePassword";
    pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub struct State {
        pub database: Data<dyn Database>,
        pub token_store: Data<dyn TokenStore>,
        pub config: Data<Config>,
    }

    pub fn get_state() -> State {
        State {
            database: get_database(),
            token_store: get_token_store(),
            config: get_config(),
        }
    }

    pub fn get_config() -> Data<Config> {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};

        Data::from(Arc::new(Config {
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0),
            secrets: rand::random(),
            google: Some(houseflow_config::server::google::Config {
                client_id: "some-client-id".to_string(),
                client_secret: "some-client-secret".to_string(),
                project_id: "some-project-id".to_string(),
            }),
            database_path: std::path::PathBuf::new(),
            tokens_path: std::path::PathBuf::new(),
        }))
    }

    pub fn get_database() -> Data<dyn houseflow_db::Database> {
        Data::from(Arc::new(SqliteDatabase::new_in_memory().unwrap()) as Arc<dyn Database>)
    }

    pub fn get_token_store() -> Data<dyn TokenStore> {
        let path =
            std::env::temp_dir().join(format!("houseflow-server_test-{}", rand::random::<u32>()));
        Data::from(
            Arc::new(token_store::sled::TokenStore::new_temporary(path).unwrap())
                as Arc<dyn TokenStore>,
        )
    }

    pub fn get_user() -> User {
        let id: UserID = rand::random();
        User {
            id: id.clone(),
            username: format!("john-{}", id.clone()),
            email: format!("john-{}@example.com", id.clone()),
            password_hash: PASSWORD_HASH.into(),
        }
    }

    pub fn get_structure() -> Structure {
        Structure {
            id: rand::random(),
            name: "test-home".to_string(),
        }
    }

    pub fn get_room(structure: &Structure) -> Room {
        Room {
            id: rand::random(),
            structure_id: structure.id.clone(),
            name: "test-garage".to_string(),
        }
    }

    pub fn get_device(room: &Room) -> Device {
        use semver::Version;

        Device {
            id: rand::random(),
            room_id: room.id.clone(),
            password_hash: PASSWORD_HASH.into(),
            device_type: DeviceType::Gate,
            traits: vec![],
            name: String::from("SuperTestingGate"),
            will_push_state: true,
            model: String::from("gate-1200"),
            hw_version: Version::new(1, 0, 0),
            sw_version: Version::new(1, 0, 1),
            attributes: Default::default(),
        }
    }
}
