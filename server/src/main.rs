use actix_web::{web, HttpServer};
use houseflow_config::{server::Config, Config as _};
use houseflow_db::{sqlite::Database as SqliteDatabase, Database};
use houseflow_server::{Sessions, SledTokenStore, TokenStore};
use std::sync::Arc;

pub struct RootSpanBuilder;

impl tracing_actix_web::RootSpanBuilder for RootSpanBuilder {
    fn on_request_start(request: &actix_web::dev::ServiceRequest) -> tracing::Span {
        let connection_info = request.connection_info();

        tracing::info_span!(
            "HttpRequest",
            method = %request.method(),
            path = %request.path(),
            client_address = %connection_info.remote_addr().unwrap_or("unknown"),
            host = %connection_info.host(),
        )
    }

    fn on_request_end<B>(
        span: tracing::Span,
        outcome: &Result<actix_web::dev::ServiceResponse<B>, actix_web::Error>,
    ) {
        let handle_error = |error: &actix_web::Error| {
            let response_error = error.as_response_error();
            span.record(
                "exception.message",
                &tracing::field::display(response_error),
            );
            span.record("exception.details", &tracing::field::debug(response_error));
            let status_code = response_error.status_code();
            span.record("http.status_code", &status_code.as_u16());
        };

        match &outcome {
            Ok(response) => {
                if let Some(error) = response.response().error() {
                    handle_error(error);
                } else {
                    span.record("status_code", &response.response().status().as_u16());
                }
            }
            Err(error) => handle_error(error),
        };
    }
}

#[actix_web::main]
async fn main() {
    const HIDE_TIMESTAMP_ENV: &str = "HOUSEFLOW_SERVER_HIDE_TIMESTAMP";

    houseflow_config::init_logging(std::env::var_os(HIDE_TIMESTAMP_ENV).is_some());
    let config_path = std::env::var("HOUSEFLOW_SERVER_CONFIG")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| Config::default_path());

    let config = Config::read(config_path).expect("cannot load server config");
    let config = web::Data::new(config);
    let token_store = SledTokenStore::new(&config.tokens_path).expect("cannot open token store");
    let token_store = web::Data::from(Arc::new(token_store) as Arc<dyn TokenStore>);

    let database = SqliteDatabase::new(&config.database_path).expect("cannot open database");
    let database = web::Data::from(Arc::new(database) as Arc<dyn Database>);
    let sessions = web::Data::new(Sessions::default());
    let config_cloned = config.clone();
    let server = HttpServer::new(move || {
        actix_web::App::new()
            .wrap(tracing_actix_web::TracingLogger::<RootSpanBuilder>::new())
            .configure(|cfg| {
                houseflow_server::configure(
                    cfg,
                    token_store.clone(),
                    database.clone(),
                    config_cloned.clone(),
                    sessions.clone(),
                )
            })
    })
    .shutdown_timeout(5);
    let address = (
        config.hostname.to_string(),
        houseflow_config::defaults::server_port(),
    );

    let server = if let Some(tls) = &config.tls {
        let address_tls = (
            config.hostname.to_string(),
            houseflow_config::defaults::server_port_tls(),
        );
        tracing::info!("Starting server with TLS");
        let mut rustls_config = rustls::ServerConfig::new(rustls::NoClientAuth::new());

        let certificate = &mut std::io::BufReader::new(
            std::fs::File::open(&tls.certificate_path).expect("read certificate fail"),
        );
        let private_key = &mut std::io::BufReader::new(
            std::fs::File::open(&tls.private_key_path).expect("read private key fail"),
        );
        let certificate_chain = rustls::internal::pemfile::certs(certificate).unwrap();
        let keys = rustls::internal::pemfile::pkcs8_private_keys(private_key).unwrap();
        rustls_config
            .set_single_cert(certificate_chain, keys.into_iter().next().unwrap())
            .unwrap();
        server
            .bind(address)
            .expect("bind server port failed")
            .bind_rustls(address_tls, rustls_config)
            .expect("bind TLS server port failed")
            .run()
    } else {
        tracing::info!("Starting server without TLS");
        server.bind(address).expect("bind server port failed").run()
    };
    server.await.expect("run server fail");
}
