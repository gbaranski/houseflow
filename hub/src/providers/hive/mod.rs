mod connect;
mod session;
mod provider;

use crate::providers::EventSender;
pub use provider::HiveProvider;


pub fn app(
    hive_provider: provider::Address,
    events: EventSender,
) -> axum::Router {
    use axum::routing::get;

    axum::Router::new()
        .route("/websocket", get(connect::websocket_handler))
        .layer(axum::AddExtensionLayer::new(hive_provider))
        .layer(axum::AddExtensionLayer::new(events))
}