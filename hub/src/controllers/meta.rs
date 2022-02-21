pub fn app(master_provider: providers::MasterHandle) -> axum::Router {
    use axum::routing::get;
    use axum::routing::post;
    use axum::AddExtensionLayer;

    axum::Router::new()
        .route(
            "/characteristic/:accessory_id/:service_name/:characteristic_name",
            get(read_characteristic),
        )
        .route(
            "/characteristic/:accessory_id/:service_name",
            post(write_characteristic),
        )
        .layer(AddExtensionLayer::new(master_provider))
}

use crate::providers;
use crate::providers::ProviderExt;
use axum::extract::Extension;
use axum::extract::Json;
use axum::extract::Path;
use houseflow_types::accessory;
use houseflow_types::accessory::characteristics::Characteristic;
use houseflow_types::accessory::characteristics::CharacteristicName;
use houseflow_types::accessory::services::ServiceName;
use houseflow_types::hub;

async fn read_characteristic(
    Extension(master_provider): Extension<providers::MasterHandle>,
    Path((accessory_id, service_name, characteristic_name)): Path<(
        accessory::ID,
        ServiceName,
        CharacteristicName,
    )>,
) -> Result<Json<Characteristic>, hub::Error> {
    let characteristic = master_provider
        .read_characteristic(accessory_id, service_name, characteristic_name)
        .await?;
    Ok(Json(characteristic))
}

async fn write_characteristic(
    Extension(master_provider): Extension<providers::MasterHandle>,
    Path((accessory_id, service_name)): Path<(accessory::ID, ServiceName)>,
    Json(characteristic): Json<Characteristic>,
) -> Result<(), hub::Error> {
    master_provider
        .write_characteristic(accessory_id, service_name, characteristic)
        .await?;
    Ok(())
}
