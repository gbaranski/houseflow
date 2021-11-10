mod extractors;

mod auth;
pub mod clerk;
mod fulfillment;
mod lighthouse;
pub mod mailer;
mod oauth;

use axum::Router;
use dashmap::DashMap;
use houseflow_config::server::Config;
use houseflow_types::device;
use mailer::Mailer;
use std::sync::Arc;

async fn health_check() -> &'static str {
    "I'm alive!"
}

#[derive(Clone)]
pub struct State {
    pub clerk: Arc<dyn clerk::Clerk>,
    pub mailer: Arc<dyn Mailer>,
    pub config: Arc<Config>,
    pub sessions: Arc<DashMap<device::ID, lighthouse::Session>>,
}

pub fn app(state: State) -> Router<axum::routing::BoxRoute> {
    use axum::handler::get;
    use axum::handler::post;
    use http::Request;
    use http::Response;
    use hyper::Body;
    use std::time::Duration;
    use tower_http::trace::TraceLayer;
    use tracing::Span;

    Router::new()
        .route("/health_check", get(health_check))
        .nest(
            "/auth",
            Router::new()
                .route("/login", post(auth::login::handle))
                .route("/refresh", post(auth::refresh::handle))
                .route("/whoami", get(auth::whoami::handle))
                .boxed(),
        )
        .nest(
            "/oauth",
            Router::new()
                .route("/authorize", get(oauth::authorize::handle))
                .route("/login", post(oauth::login::handle))
                .route("/google_login", post(oauth::google_login::handle))
                .route("/token", post(oauth::token::handle)),
        )
        .nest(
            "/fulfillment",
            Router::new()
                .nest(
                    "/internal",
                    Router::new()
                        .route("/execute", post(fulfillment::internal::execute::handle))
                        .route("/query", post(fulfillment::internal::query::handle))
                        .route("/sync", get(fulfillment::internal::sync::handle)),
                )
                .route("/google-home", post(fulfillment::ghome::handle)),
        )
        .nest(
            "/lighthouse",
            Router::new().route("/ws", get(lighthouse::connect::handle)),
        )
        .layer(axum::AddExtensionLayer::new(state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    tracing::debug_span!(
                        "Request",
                        status_code = tracing::field::Empty,
                        ms = tracing::field::Empty,
                        path = tracing::field::display(request.uri().path()),
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
    use super::mailer::fake::Mailer as FakeMailer;
    use super::State;
    use crate::clerk::sled::Clerk;
    use axum::extract;
    use houseflow_config::server::Config;
    use houseflow_config::server::Email;
    use houseflow_config::server::GoogleLogin;
    use houseflow_config::server::Network;
    use houseflow_config::server::Secrets;
    use houseflow_types::code::VerificationCode;
    use std::sync::Arc;
    use tokio::sync::mpsc;
    use url::Url;

    use houseflow_types::device;
    use houseflow_types::permission;
    use houseflow_types::room;
    use houseflow_types::structure;
    use houseflow_types::user;

    use device::Device;
    use permission::Permission;
    use room::Room;
    use structure::Structure;
    use user::User;

    pub fn get_state(
        tx: &mpsc::UnboundedSender<VerificationCode>,
        structures: Vec<Structure>,
        rooms: Vec<Room>,
        devices: Vec<Device>,
        permissions: Vec<Permission>,
        users: Vec<User>,
    ) -> extract::Extension<State> {
        let config = Config {
            network: Network::default(),
            secrets: Secrets {
                refresh_key: String::from("refresh-key"),
                access_key: String::from("access-key"),
                authorization_code_key: String::from("authorization-code-key"),
            },
            tls: None,
            email: Email {
                from: String::new(),
                url: Url::parse("smtp://localhost").unwrap(),
            },
            google: Some(houseflow_config::server::Google {
                client_id: String::from("client-id"),
                client_secret: String::from("client-secret"),
                project_id: String::from("project-id"),
            }),
            google_login: Some(GoogleLogin {
                client_id: String::from("google-login-client-id"),
            }),
            structures,
            rooms,
            devices,
            users,
            permissions,
        };

        let sessions = Default::default();
        let clerk_path =
            std::env::temp_dir().join(format!("houseflow-clerk-test-{}", rand::random::<u32>()));

        extract::Extension(State {
            config: Arc::new(config),
            mailer: Arc::new(FakeMailer::new(tx.clone())),
            sessions,
            clerk: Arc::new(Clerk::new_temporary(clerk_path).unwrap()),
        })
    }

    pub fn get_user() -> User {
        let id = user::ID::new_v4();
        User {
            id: id.clone(),
            username: format!("john-{}", id.clone()),
            email: format!("john-{}@example.com", id.clone()),
            admin: false,
        }
    }

    #[allow(dead_code)]
    pub fn get_structure() -> Structure {
        Structure {
            id: structure::ID::new_v4(),
            name: "test-home".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get_room(structure: &Structure) -> Room {
        Room {
            id: room::ID::new_v4(),
            structure_id: structure.id.clone(),
            name: "test-garage".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get_device(room: &Room) -> Device {
        use semver::Version;

        Device {
            id: device::ID::new_v4(),
            room_id: room.id.clone(),
            password_hash: Some("$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU".into()),
            device_type: device::Type::Gate,
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
