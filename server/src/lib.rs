pub mod clerk;
pub mod mailer;

pub mod controllers;
pub mod providers;

pub mod auth;

pub mod extractors;

use anyhow::Context;
use houseflow_config::dynamic;
use houseflow_config::server::{Config, Network as NetworkConfig, Tls as TlsConfig};
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
pub struct State {
    config: dynamic::Config<Config>,

    #[allow(dead_code)]
    mailer: mailer::Handle,
    #[allow(dead_code)]
    clerk: Arc<dyn clerk::Clerk>,

    provider: providers::Handle,
}

impl State {
    pub fn new(
        config: Config,
        mailer: mailer::Handle,
        clerk: Arc<dyn clerk::Clerk>,
        provider: providers::Handle,
    ) -> Self {
        Self {
            config: dynamic::Config::new(config),
            mailer,
            clerk,
            provider,
        }
    }
}

async fn health_check() -> &'static str {
    "I'm alive!"
}

fn app(state: State) -> axum::Router {
    use axum::routing::get;
    use axum::Router;

    Router::new()
        .route("/health-check", get(health_check))
        .nest("/auth", auth::app())
        .nest(
            "/controllers",
            Router::new().nest("/meta", controllers::meta::app()),
        )
        .nest(
            "/providers",
            Router::new().nest("/lighthouse", providers::lighthouse::app()),
        )
        .layer(axum::AddExtensionLayer::new(state))
}

pub async fn http_server(state: State) {
    let NetworkConfig { address, port, .. } = state.config.get().network;
    let address = SocketAddr::new(address, port);
    let app = app(state);
    let fut = axum_server::bind(address).serve(app.into_make_service());
    tracing::info!("serving http server on {}", address);
    fut.await.unwrap()
}

pub async fn https_server(
    state: State,
    TlsConfig {
        address,
        port,
        certificate,
        private_key,
    }: TlsConfig,
) {
    let app = app(state);
    let rustls_config =
        axum_server::tls_rustls::RustlsConfig::from_pem_file(certificate, private_key)
            .await
            .context("invalid TLS configuration")
            .unwrap();
    let address = SocketAddr::new(address, port);
    let fut = axum_server::bind_rustls(address, rustls_config).serve(app.into_make_service());
    tracing::info!("serving http server on {}", address);
    fut.await.unwrap()
}

#[cfg(test)]
mod test_utils {
    use crate::clerk;
    use crate::clerk::Clerk;
    use crate::mailer;
    use crate::providers::FakeProvider;
    use crate::State;
    use axum::extract;
    use houseflow_config::server::Config;
    use houseflow_config::server::Controllers;
    use houseflow_config::server::Email;
    use houseflow_config::server::GoogleLogin;
    use houseflow_config::server::Logins;
    use houseflow_config::server::Network;
    use houseflow_config::server::Providers;
    use houseflow_config::server::Secrets;
    use houseflow_types::code::VerificationCode;
    use std::sync::Arc;
    use tokio::sync::mpsc;

    use houseflow_types::permission;
    use houseflow_types::structure;
    use houseflow_types::user;

    use permission::Permission;
    use structure::Structure;
    use url::Url;
    use user::User;

    #[derive(Default)]
    pub struct GetState {
        pub mailer_tx: Option<mpsc::UnboundedSender<(lettre::Address, VerificationCode)>>,
        pub structures: Vec<Structure>,
        pub permissions: Vec<Permission>,
        pub users: Vec<User>,
    }

    pub fn get_state(
        GetState {
            mailer_tx,
            structures,
            permissions,
            users,
        }: GetState,
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
            controllers: Controllers { meta: None },
            providers: Providers { lighthouse: None },
            logins: Logins {
                google: Some(GoogleLogin {
                    client_id: String::from("google-login-client-id"),
                }),
            },
            structures,
            users,
            permissions,
        };

        let clerk_path =
            std::env::temp_dir().join(format!("houseflow-clerk-test-{}", rand::random::<u32>()));
        let clerk = Arc::new(clerk::Sled::new_temporary(clerk_path).unwrap());
        let provider = FakeProvider::create();
        let mailer = mailer::Fake::create(
            mailer_tx.unwrap_or_else(|| tokio::sync::mpsc::unbounded_channel().0),
        );
        let state = State::new(config, mailer, clerk as Arc<dyn Clerk>, provider);
        extract::Extension(state)
    }

    pub fn get_user() -> User {
        let id = user::ID::new_v4();
        User {
            id: id.clone(),
            username: format!("john-{}", id.clone()),
            email: lettre::Address::new("john", "email.com").unwrap(),
            admin: false,
        }
    }
}
