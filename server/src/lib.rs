pub mod clerk;
pub mod mailer;

pub mod controllers;
pub mod providers;

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
