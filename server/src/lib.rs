mod extractors;

mod blacklist;

mod auth;
mod fulfillment;
mod lighthouse;
mod oauth;

pub use blacklist::sled::TokenBlacklist as SledTokenBlacklist;
pub use blacklist::TokenBlacklist;

use axum::AddExtensionLayer;
use axum::Router;
use dashmap::DashMap;
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::errors::AuthError;
use houseflow_types::DeviceID;

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
    pub token_blacklist: Arc<dyn TokenBlacklist>,
    pub database: Arc<dyn Database>,
    pub config: Arc<Config>,
    pub sessions: DashMap<DeviceID, lighthouse::Session>,
}

use tokio::net::TcpListener;

pub async fn run_tls(
    address: &std::net::SocketAddr,
    state: State,
    tls_config: Arc<tokio_rustls::rustls::ServerConfig>,
) -> Result<(), tokio::io::Error> {
    use tokio_rustls::TlsAcceptor;

    let acceptor = TlsAcceptor::from(tls_config);
    let listener = TcpListener::bind(address).await?;
    let app = app(state);
    loop {
        let (stream, address) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let app = app.clone().layer(AddExtensionLayer::new(address));
        tokio::spawn(async move {
            if let Ok(stream) = acceptor.accept(stream).await {
                match hyper::server::conn::Http::new()
                    .serve_connection(stream, app)
                    .with_upgrades()
                    .await
                {
                    Ok(_) => (),
                    Err(err) => tracing::warn!("accept connection error: {}", err),
                };
            }
        });
    }
}

pub async fn run(address: &std::net::SocketAddr, state: State) -> Result<(), tokio::io::Error> {
    let listener = TcpListener::bind(address).await?;
    let app = app(state);
    loop {
        let (stream, address) = listener.accept().await?;
        let app = app.clone().layer(AddExtensionLayer::new(address));
        tokio::spawn(async move {
            match hyper::server::conn::Http::new()
                .serve_connection(stream, app)
                .with_upgrades()
                .await
            {
                Ok(_) => (),
                Err(err) => tracing::warn!("accept connection error: {}", err),
            };
        });
    }
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
                .route("/logout", post(auth::logout::handle))
                .route("/register", post(auth::register::handle))
                .route("/refresh", post(auth::refresh::handle))
                .route("/whoami", get(auth::whoami::handle))
                .boxed(),
        )
        .nest(
            "/oauth",
            Router::new()
                .route("/authorize", get(oauth::authorize::handle))
                .route("/login", post(oauth::login::handle))
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
    use super::SledTokenBlacklist;
    use super::State;
    use axum::extract;
    use houseflow_config::defaults;
    use houseflow_config::server::Config;
    use houseflow_config::server::Network;
    use houseflow_config::server::Secrets;
    use houseflow_db::sqlite::Database as SqliteDatabase;
    use houseflow_types::Device;
    use houseflow_types::DeviceType;
    use houseflow_types::Room;
    use houseflow_types::Structure;
    use houseflow_types::User;
    use houseflow_types::UserID;
    use std::sync::Arc;

    pub const PASSWORD: &str = "SomePassword";
    pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub fn get_state() -> extract::Extension<State> {
        let database = SqliteDatabase::new_in_memory().unwrap();
        let token_blacklist_path =
            std::env::temp_dir().join(format!("houseflow-server_test-{}", rand::random::<u32>()));
        let token_blacklist = SledTokenBlacklist::new_temporary(token_blacklist_path).unwrap();
        let config = Config {
            network: Network {
                address: defaults::server_address(),
            },
            secrets: Secrets {
                refresh_key: String::from("refresh-key"),
                access_key: String::from("access-key"),
                authorization_code_key: String::from("authorization-code-key"),
            },
            tls: None,
            google: Some(houseflow_config::server::Google {
                client_id: String::from("client-id"),
                client_secret: String::from("client-secret"),
                project_id: String::from("project-id"),
            }),
            structures: vec![],
            rooms: vec![],
            devices: vec![],
            permissions: vec![],
        };

        let sessions = Default::default();

        extract::Extension(State {
            database: Arc::new(database),
            token_blacklist: Arc::new(token_blacklist),
            config: Arc::new(config),
            sessions,
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
