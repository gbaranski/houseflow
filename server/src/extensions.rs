use axum::extract::Extension;
use houseflow_config::dynamic;
use houseflow_config::server::Config as ServerConfig;
use std::sync::Arc;

pub type Config = Extension<dynamic::Config<ServerConfig>>;
pub type Clerk = Extension<Arc<dyn crate::clerk::Clerk>>;
pub type MasterMailer = Extension<crate::mailer::MasterHandle>;
