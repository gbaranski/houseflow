pub mod controllers;
pub mod providers;

use std::net::SocketAddr;

use houseflow_config::hub::Config;
use houseflow_config::hub::Controllers;
use houseflow_config::hub::Providers;

pub async fn run(config: Config) -> Result<(), anyhow::Error> {
    use acu::MasterExt;
    use axum::routing::get;
    use axum::Router;

    let router = Router::new().route("/health-check", get(health_check));

    let master_controller = controllers::MasterHandle::new();
    let master_provider = providers::MasterHandle::new();

    let controller_router = {
        let Controllers {
            hap,
            lighthouse,
            meta,
        } = config.controllers;
        let mut router = Router::new();
        if let Some(hap) = hap {
            let handle = controllers::hap::new(hap, master_provider.clone()).await?;
            master_controller.push(handle).await;
        }
        if let Some(lighthouse) = lighthouse {
            let handle =
                controllers::lighthouse::new(lighthouse, config.hub.id, master_provider.clone())
                    .await?;
            master_controller.push(handle).await;
        }
        if let Some(_meta) = meta {
            let app = controllers::meta::app(master_provider.clone());
            router = router.nest("/meta", app);
        }
        router
    };

    let provider_router = {
        let Providers { hive, mijia } = config.providers;
        let mut router = Router::new();
        if let Some(hive) = hive {
            let handle =
                providers::hive::new(hive, master_controller.clone(), config.accessories.clone());
            let app = providers::hive::app(master_controller.clone().into(), handle.clone());
            router = router.nest("/hive", app);
            master_provider.push(handle.into()).await;
        }
        if let Some(mijia) = mijia {
            let handle =
                providers::mijia::new(mijia, master_controller.clone(), config.accessories.clone())
                    .await?;
            master_provider.push(handle).await;
        }
        router
    };

    let router = router
        .nest("/controller", controller_router)
        .nest("/provider", provider_router);

    let address = SocketAddr::new(config.network.address, config.network.port);
    let fut = axum_server::bind(address).serve(router.clone().into_make_service());
    tracing::info!("serving http server on {}", address);
    fut.await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "I'm alive!"
}

// impl Hub {
//     pub async fn new(
//         controller: controllers::MasterHandle,
//         provider: providers::MasterHandle,
//     ) -> Result<Self, anyhow::Error> {
//         Ok(Self {
//             controller,
//             provider,
//         })
//     }

//     pub async fn run(self) -> Result<(), anyhow::Error> {
//         tokio::select! {
//             _ = self.controller.wait_for_stop() => {
//                 tracing::info!("controller {} has stopped", self.controller.name());
//             },
//             _ = self.provider.wait_for_stop() => {
//                 tracing::info!("provider {} has stopped", self.provider.name());
//             },
//         }

//         Ok(())
//     }
// }
