pub mod auth;
pub mod clerk;
pub mod controllers;
pub mod extensions;
pub mod extractors;
pub mod mailer;
pub mod providers;

use acu::MasterExt;
use anyhow::Context;
use houseflow_config::dynamic;
use houseflow_config::server::Config;
use houseflow_config::server::Network as NetworkConfig;
use houseflow_config::server::Tls as TlsConfig;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct ArgMailers {
    pub dummy: Option<mailer::dummy::Handle>,
    pub smtp: Option<mailer::smtp::Handle>,
}

type CreateFn<M, H> = Option<Box<dyn FnOnce(M) -> H>>;

type ProviderCreateFn<H> = CreateFn<controllers::MasterHandle, H>;

pub struct ArgProviders {
    pub dummy: ProviderCreateFn<providers::dummy::Handle>,
    pub lighthouse: ProviderCreateFn<providers::lighthouse::LighthouseProviderHandle>,
}

type ControllerCreateFn<H> = CreateFn<providers::MasterHandle, H>;

pub struct ArgControllers {
    // pub dummy: Option<controllers::dum>,
    pub meta: ControllerCreateFn<controllers::meta::Handle>,
}

pub struct Arg {
    pub config: dynamic::Config<Config>,
    pub clerk: Arc<dyn clerk::Clerk>,
    pub mailers: ArgMailers,
    pub providers: ArgProviders,
    pub controllers: ArgControllers,
}

pub struct Server {
    router: axum::Router,
    config: dynamic::Config<Config>,
}

impl Server {
    pub async fn new(
        Arg {
            config,
            clerk,
            mailers,
            providers,
            controllers,
        }: Arg,
    ) -> Self {
        use axum::routing::get;
        use axum::Router;

        let router = Router::new()
            .route("/health-check", get(health_check))
            .nest("/auth", auth::app());

        let master_controller = controllers::MasterHandle::new();
        let master_provider = providers::MasterHandle::new();

        let controller_router = async {
            let ArgControllers { meta } = controllers;
            let mut router = Router::new();
            if let Some(meta) = meta {
                let meta = meta(master_provider.clone());
                master_controller.push(meta.clone()).await;
                router = router.nest("/meta", controllers::meta::app(meta));
            }
            router
        }
        .await;

        let provider_router = async {
            let ArgProviders { dummy, lighthouse } = providers;
            let mut router = Router::new();
            if let Some(dummy) = dummy {
                let dummy = dummy(master_controller.clone());
                master_provider.push(dummy.clone()).await;
            }
            if let Some(lighthouse) = lighthouse {
                let lighthouse = lighthouse(master_controller.clone());
                master_provider.push(lighthouse.clone().into()).await;
                router = router.nest("/lighthouse", providers::lighthouse::app(lighthouse));
            }
            router
        }
        .await;

        let master_mailer = async {
            let ArgMailers { dummy, smtp } = mailers;
            let master = mailer::MasterHandle::new();
            if let Some(dummy) = dummy {
                master.push(dummy).await;
            }
            if let Some(smtp) = smtp {
                master.push(smtp).await;
            }
            master
        }
        .await;

        let router = router
            .nest("/controller", controller_router)
            .nest("/provider", provider_router)
            .layer(axum::AddExtensionLayer::new(config.clone()))
            .layer(axum::AddExtensionLayer::new(clerk))
            .layer(axum::AddExtensionLayer::new(master_controller))
            .layer(axum::AddExtensionLayer::new(master_provider))
            .layer(axum::AddExtensionLayer::new(master_mailer));

        Self { router, config }
    }

    pub async fn run(&self) {
        let http_future = http_server(self.router.clone(), self.config.get().network.to_owned());
        if let Some(tls) = self.config.get().tls.to_owned() {
            let https_future = https_server(self.router.clone(), tls);
            tokio::select! {
                result = http_future => {
                    tracing::info!("http server finished with {:?}", result);
                }
                result = https_future => {
                    tracing::info!("https server finished with {:?}", result);
                }
            }
        }
    }
}

#[tracing::instrument(err)]
async fn http_server(
    router: axum::Router,
    NetworkConfig {
        address,
        port,
        base_url: _,
    }: NetworkConfig,
) -> Result<(), std::io::Error> {
    let address = SocketAddr::new(address, port);
    let fut = axum_server::bind(address).serve(router.clone().into_make_service());
    tracing::info!("serving http server on {}", address);
    fut.await
}

#[tracing::instrument(err)]
async fn https_server(
    router: axum::Router,
    TlsConfig {
        address,
        port,
        certificate,
        private_key,
    }: TlsConfig,
) -> Result<(), std::io::Error> {
    let rustls_config =
        axum_server::tls_rustls::RustlsConfig::from_pem_file(certificate, private_key)
            .await
            .context("invalid TLS configuration")
            .unwrap();
    let address = SocketAddr::new(address, port);
    let fut =
        axum_server::bind_rustls(address, rustls_config).serve(router.clone().into_make_service());
    tracing::info!("serving https server on {}", address);
    fut.await
}

async fn health_check() -> &'static str {
    "I'm alive!"
}

#[cfg(test)]
mod test_utils {
    use crate::*;
    use axum::extract::Extension;
    use houseflow_config::server::*;
    use houseflow_types::code::VerificationCode;
    use houseflow_types::permission;
    use houseflow_types::structure;
    use houseflow_types::user;
    use permission::Permission;
    use std::sync::Arc;
    use structure::Structure;
    use tokio::sync::mpsc;
    use user::User;

    #[derive(Default)]
    pub struct GetState {
        pub mailer_tx: Option<mpsc::UnboundedSender<(lettre::Address, VerificationCode)>>,
        pub structures: Vec<Structure>,
        pub permissions: Vec<Permission>,
        pub users: Vec<User>,
    }

    #[derive(Default)]
    pub struct GetConfig {
        pub structures: Vec<Structure>,
        pub permissions: Vec<Permission>,
        pub users: Vec<User>,
    }

    pub async fn get_config(
        GetConfig {
            structures,
            permissions,
            users,
        }: GetConfig,
    ) -> extensions::Config {
        let config = Config {
            network: Network::default(),
            secrets: Secrets {
                refresh_key: String::from("refresh-key"),
                access_key: String::from("access-key"),
                authorization_code_key: String::from("authorization-code-key"),
            },
            tls: None,
            mailers: Mailers {
                smtp: None,
                dummy: Some(mailers::Dummy {}),
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
        let config = houseflow_config::dynamic::Config::new(config);
        Extension(config)
    }

    #[derive(Default)]
    pub struct GetClerk {}

    pub async fn get_clerk(GetClerk {}: GetClerk) -> extensions::Clerk {
        let clerk_path =
            std::env::temp_dir().join(format!("houseflow-clerk-test-{}", rand::random::<u32>()));
        let clerk = Arc::new(crate::clerk::Sled::new_temporary(clerk_path).unwrap());
        Extension(clerk)
    }

    #[derive(Default)]
    pub struct GetMasterMailer {
        pub tx: Option<mpsc::UnboundedSender<(lettre::Address, VerificationCode)>>,
    }

    pub async fn get_master_mailer(
        GetMasterMailer { tx }: GetMasterMailer,
    ) -> extensions::MasterMailer {
        let master = mailer::MasterHandle::new();
        let dummy =
            mailer::dummy::new(tx.unwrap_or_else(|| tokio::sync::mpsc::unbounded_channel().0));
        master.push(dummy).await;
        Extension(master)
    }

    // pub async fn get_state(
    //     GetState {
    //         mailer_tx,
    //         structures,
    //         permissions,
    //         users,
    //     }: GetState,
    // ) -> extract::Extension<State> {
    //     let config = Config {
    //         network: Network::default(),
    //         secrets: Secrets {
    //             refresh_key: String::from("refresh-key"),
    //             access_key: String::from("access-key"),
    //             authorization_code_key: String::from("authorization-code-key"),
    //         },
    //         tls: None,
    //         mailers: Mailers {
    //             smtp: None,
    //             dummy: Some(mailers::Dummy {}),
    //         },
    //         controllers: Controllers { meta: None },
    //         providers: Providers { lighthouse: None },
    //         logins: Logins {
    //             google: Some(GoogleLogin {
    //                 client_id: String::from("google-login-client-id"),
    //             }),
    //         },
    //         structures,
    //         users,
    //         permissions,
    //     };
    //
    //     let clerk_path =
    //         std::env::temp_dir().join(format!("houseflow-clerk-test-{}", rand::random::<u32>()));
    //     let clerk = Arc::new(clerk::Sled::new_temporary(clerk_path).unwrap());
    //     let provider = {
    //         let master = providers::MasterHandle::new();
    //         let dummy = providers::dummy::new();
    //         master.push(dummy).await;
    //         master
    //     };
    //     let mailer = {
    //         let master = mailer::MasterHandle::new();
    //         let dummy = mailer::dummy::new(
    //             mailer_tx.unwrap_or_else(|| tokio::sync::mpsc::unbounded_channel().0),
    //         );
    //         master.push(dummy).await;
    //         master
    //     };
    //     let state = State::new(config, mailer, clerk as Arc<dyn Clerk>, provider);
    //     extract::Extension(state)
    // }

    pub fn get_user() -> User {
        let id = user::ID::new_v4();
        User {
            id: id,
            username: format!("john-{}", id.clone()),
            email: lettre::Address::new("john", "email.com").unwrap(),
            admin: false,
        }
    }
}
