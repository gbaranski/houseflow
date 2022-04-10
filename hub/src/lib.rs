pub mod controllers;
pub mod providers;

use std::net::SocketAddr;
use std::sync::Arc;

use arc_swap::ArcSwap;
use houseflow_config::hub::Accessory;
use houseflow_config::hub::Config;
use houseflow_config::hub::Controllers;
use houseflow_config::hub::Providers;

use paste::paste;
use std::stringify;

macro_rules! optional_component {
    ($prefix:expr, $name:expr, $body:tt) => {
        paste! {
            #[cfg(feature = $prefix "-" $name)]
            if let Some($name) = $name {
                $body
            }
            #[cfg(not(feature = $prefix "-" $name))]
            if $name.is_some() {
                let name = stringify!($name);
                let prefix = $prefix;
                tracing::warn!("houseflow-hub is not compiled with `{prefix}-{name}` feature enabled, but `{prefix}.{name}` is set in the config file. Please either remove it or enable the feature.");
            }
        }
    };
}

macro_rules! optional_controller {
    ($name:ident, $body:tt) => {
        optional_component!("controllers", $name, $body)
    };
}

macro_rules! optional_provider {
    ($name:ident, $body:tt) => {
        optional_component!("providers", $name, $body)
    };
}

pub type ConfiguredAccessories = Arc<ArcSwap<Vec<Accessory>>>;

pub async fn run(config: Config) -> Result<(), anyhow::Error> {
    #[allow(unused_imports)]
    use acu::MasterExt;
    use axum::routing::get;
    use axum::Router;

    let router = Router::new().route("/health-check", get(health_check));

    #[allow(unused_variables)]
    let configured_accessories = Arc::new(ArcSwap::from(Arc::new(config.accessories)));

    #[allow(unused_variables)]
    let master_controller = controllers::MasterHandle::new();
    #[allow(unused_variables)]
    let master_provider = providers::MasterHandle::new();

    let controller_router = {
        let Controllers {
            hap,
            lighthouse,
            meta,
        } = config.controllers;

        #[allow(unused_mut)]
        let mut router = Router::new();

        optional_controller!(hap, {
            let handle = controllers::hap::new(hap, master_provider.clone()).await?;
            master_controller.push(handle).await;
        });

        optional_controller!(lighthouse, {
            let handle =
                controllers::lighthouse::new(lighthouse, config.hub.id, master_provider.clone())
                    .await?;
            master_controller.push(handle).await;
        });

        optional_controller!(meta, {
            let _meta = meta;
            let app = controllers::meta::app(master_provider.clone());
            router = router.nest("/meta", app);
        });

        router
    };

    let provider_router = {
        let Providers { hive, mijia } = config.providers;
        #[allow(unused_mut)]
        let mut router = Router::new();

        optional_provider!(hive, {
            let server = providers::hive::new(
                hive,
                master_controller.clone(),
                configured_accessories.clone(),
            );

            let handle = providers::Handle {
                sender: acu::Sender::new_from_mpsc(server.clone().into(), providers::Name::Hive),
            };
            let app = providers::hive::app(
                server,
                configured_accessories.clone(),
                master_provider.clone(),
            );
            router = router.nest("/hive", app);
            master_provider.push(handle).await;
        });
        optional_provider!(mijia, {
            let handle = providers::mijia::new(
                mijia,
                master_controller.clone(),
                configured_accessories.clone(),
            )
            .await?;
            master_provider.push(handle).await;
        });

        router
    };

    let router = router
        .nest("/controller", controller_router)
        .nest("/provider", provider_router);

    let address = SocketAddr::new(config.network.address, config.network.port);
    let fut = axum_server::bind(address).serve(
        router
            .clone()
            .into_make_service_with_connect_info::<SocketAddr>(),
    );
    tracing::info!("serving http server on {}", address);
    fut.await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "I'm alive!"
}
