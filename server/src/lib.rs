mod extractors;

mod token_store;

mod auth;
mod fulfillment;
mod lighthouse;
// mod oauth;

pub use token_store::{sled::TokenStore as SledTokenStore, TokenStore};

use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{errors::AuthError, DeviceID};

use std::{collections::HashMap, sync::Mutex};
pub type Sessions = HashMap<DeviceID, lighthouse::Session>;

pub(crate) fn get_password_salt() -> [u8; 16] {
    rand::random()
}

pub(crate) fn verify_password(hash: &str, password: &str) -> Result<(), AuthError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(AuthError::InvalidPassword),
    }
}

async fn health_check() -> &'static str {
    "I'm alive!"
}

use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    pub token_store: Arc<dyn TokenStore>,
    pub database: Arc<dyn Database>,
    pub config: Arc<Config>,
    pub sessions: Arc<Mutex<Sessions>>,
}

pub async fn run(address: &std::net::SocketAddr, state: State) {
    use axum::routing::RoutingDsl;

    hyper::Server::bind(address)
        .serve(app(state).into_make_service_with_connect_info::<std::net::SocketAddr, _>())
        .await
        .expect("server error");
}

pub fn app(state: State) -> axum::routing::BoxRoute<axum::body::Body> {
    use axum::{
        prelude::{get, post, route, RoutingDsl},
        routing::nest,
    };
    use http::{Request, Response};
    use hyper::Body;
    use std::time::Duration;
    use tower_http::trace::TraceLayer;
    use tracing::Span;

    route("/health_check", get(health_check))
        .nest(
            "/auth",
            route("/login", post(auth::login::handle))
                .route("/logout", post(auth::logout::handle))
                .route("/register", post(auth::register::handle))
                .route("/refresh", post(auth::refresh::handle))
                .route("/whoami", get(auth::whoami::handle))
                .boxed(),
        )
        .nest(
            "/fulfillment",
            nest(
                "/internal",
                route("/execute", post(fulfillment::internal::execute::handle))
                    .route("/query", post(fulfillment::internal::query::handle))
                    .route("/sync", get(fulfillment::internal::sync::handle)),
            ),
        )
        .nest(
            "/lighthouse",
            route("/ws", get(lighthouse::connect::handle)),
        )
        .layer(axum::AddExtensionLayer::new(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| {
                    tracing::debug_span!(
                        "Request",
                        status_code = tracing::field::Empty,
                        latency = tracing::field::Empty,
                        user_id = tracing::field::Empty
                    )
                })
                .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
                    span.record("status_code", &tracing::field::display(response.status()));
                    span.record("ms", &tracing::field::display(latency.as_millis()));

                    tracing::debug!("response processed")
                }),
        )
        .boxed()
}

#[cfg(test)]
mod test_utils {
    use super::{token_store, Sessions, State};
    use axum::extract;
    use houseflow_config::server::{Config, Secrets};
    use houseflow_db::sqlite::Database as SqliteDatabase;
    use houseflow_types::{Device, DeviceType, Room, Structure, User, UserID};
    use std::sync::{Arc, Mutex};

    pub const PASSWORD: &str = "SomePassword";
    pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub fn get_state() -> extract::Extension<State> {
        let database = SqliteDatabase::new_in_memory().unwrap();
        let token_store_path =
            std::env::temp_dir().join(format!("houseflow-server_test-{}", rand::random::<u32>()));
        let token_store = token_store::sled::TokenStore::new_temporary(token_store_path).unwrap();
        let config = Config {
            hostname: url::Host::Domain(String::from("localhost")),
            secrets: Secrets {
                refresh_key: String::from("refresh-key"),
                access_key: String::from("access-key"),
                authorization_code_key: String::from("authorization-code-key"),
            },
            tls: None,
            google: Some(houseflow_config::server::google::Config {
                client_id: String::from("client-id"),
                client_secret: String::from("client-secret"),
                project_id: String::from("project-id"),
            }),
        };

        let sessions = Mutex::new(Sessions::new());

        extract::Extension(State {
            database: Arc::new(database),
            token_store: Arc::new(token_store),
            config: Arc::new(config),
            sessions: Arc::new(sessions),
        })
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

    #[allow(dead_code)]
    pub fn get_structure() -> Structure {
        Structure {
            id: rand::random(),
            name: "test-home".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get_room(structure: &Structure) -> Room {
        Room {
            id: rand::random(),
            structure_id: structure.id.clone(),
            name: "test-garage".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get_device(room: &Room) -> Device {
        use semver::Version;

        Device {
            id: rand::random(),
            room_id: room.id.clone(),
            password_hash: Some(PASSWORD_HASH.into()),
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
