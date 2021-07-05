mod admin;
mod auth;
mod fulfillment;
mod lighthouse;
mod token_store;

pub use token_store::{MemoryTokenStore, RedisTokenStore, TokenStore};

use actix_web::web;
use houseflow_config::server::Config;
use houseflow_db::Database;

use {
    houseflow_types::DeviceID, lighthouse::Session, std::collections::HashMap, tokio::sync::Mutex,
};
pub type Sessions = Mutex<HashMap<DeviceID, actix::Addr<Session>>>;

const AUTHORIZATION_TMPL: &str = include_str!("templates/authorization.html");

pub fn configure(
    cfg: &mut web::ServiceConfig,
    token_store: web::Data<dyn TokenStore>,
    database: web::Data<dyn Database>,
    config: web::Data<Config>,
    sessions: web::Data<Sessions>,
) {
    use tinytemplate::TinyTemplate;

    log::debug!("adding template");
    let mut tt = TinyTemplate::new();
    tt.add_template("authorization.html", AUTHORIZATION_TMPL)
        .expect("invalid authorization tmpl");

    cfg.app_data(config)
        .app_data(token_store)
        .app_data(sessions)
        .app_data(database)
        .service(
            web::scope("/admin")
                .app_data(tt)
                .service(admin::device::on_add)
                .service(admin::room::on_add)
                .service(admin::structure::on_add)
                .service(admin::user_structure::on_add),
        )
        .service(
            web::scope("/auth")
                .service(auth::on_login)
                .service(auth::on_logout)
                .service(auth::on_register)
                .service(auth::on_whoami)
                .service(
                    web::scope("/")
                        .app_data(auth::on_exchange_refresh_token_form_config())
                        .service(auth::on_exchange_refresh_token),
                ),
        )
        .service(
            web::scope("/fulfillment").service(
                web::scope("/internal")
                    .service(fulfillment::internal::on_execute)
                    .service(fulfillment::internal::on_query)
                    .service(fulfillment::internal::on_sync),
            ),
        )
        .service(web::scope("/lighthouse").service(lighthouse::on_websocket));
}

#[cfg(test)]
mod test_utils {
    use super::Config;
    use crate::{MemoryTokenStore, TokenStore};
    use houseflow_db::{Database, sqlite::Database as SqliteDatabase};
    use houseflow_types::{Device, DeviceType, Room, Structure, User, UserID};

    use actix_web::{test, web::Data, App};
    use std::sync::Arc;

    pub const PASSWORD: &str = "SomePassword";
    pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub struct State {
        pub database: Data<dyn Database>,
        pub token_store: Data<dyn TokenStore>,
        pub config: Data<Config>,
    }

    pub async fn send_request<T: serde::de::DeserializeOwned>(
        request: test::TestRequest,
    ) -> (T, State) {
        let database = get_database();
        let token_store = get_token_store();
        let config = get_config();
        let state = State {
            database,
            token_store,
            config,
        };

        let response = send_request_with_state(request, &state).await;
        (response, state)
    }

    pub async fn send_request_with_state<T: serde::de::DeserializeOwned>(
        request: test::TestRequest,
        state: &State,
    ) -> T {
        let mut app = test::init_service(App::new().configure(|cfg| {
            crate::configure(
                cfg,
                state.token_store.clone(),
                state.database.clone(),
                state.config.clone(),
                Data::new(Default::default()),
            )
        }))
        .await;
        let response = test::call_service(&mut app, request.to_request()).await;

        test::read_body_json(response).await
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
            redis: Default::default(),
            google: Some(houseflow_config::server::google::Config {
                client_id: "some-client-id".to_string(),
                client_secret: "some-client-secret".to_string(),
                project_id: "some-project-id".to_string(),
            }),
        }))
    }

    pub fn get_database() -> Data<dyn houseflow_db::Database> {
        Data::from(Arc::new(SqliteDatabase::new_in_memory().unwrap()) as Arc<dyn Database>)
    }

    pub fn get_token_store() -> Data<dyn TokenStore> {
        Data::from(Arc::new(MemoryTokenStore::new()) as Arc<dyn TokenStore>)
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
