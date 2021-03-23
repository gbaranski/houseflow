use serde::Serialize;
use uuid::Uuid;
use houseflow_db::{Database, models::{User, Device}};
use crate::Error;

// Empty module, SYNC intent doesn't have any payload
pub mod request {
}

pub mod response {
    use super::*;

    #[derive(Serialize)]
    pub struct Payload {
        /// Reflects the unique (and immutable) user ID on the agent's platform.
        #[serde(rename = "agentUserId")]
        pub user_id: Uuid,

        /// For systematic errors on SYNC
        #[serde(rename = "errorCode")]
        pub error_code: Option<String>,

        /// Detailed error which will never be presented to users but may be logged or used during development.
        #[serde(rename = "debugString")]
        pub debug_string: Option<String>,

        /// List of devices owned by the user.
        /// Zero or more devices are returned (zero devices meaning the user has no devices, or has disconnected them all).
        pub devices: Vec<Device>,
    }

    #[derive(Serialize)]
    pub struct Response {
        /// ID of the request.
        #[serde(rename = "requestId")]
        pub request_id: String,

        /// Intent response payload.
        pub payload: Payload,
    }
}

use response::Response;

pub async fn handle(db: &Database, user: &User, request_id: String) -> Result<Response, Error> {
    log::debug!("Received Sync intent from User ID: {}", user.id.to_string());
    let devices = db.get_user_devices(user.id).await?;
    
    Ok(Response {
        request_id,
        payload: response::Payload {
            user_id: user.id,
            error_code: None,
            debug_string: None,
            devices,
        }
    })
}
