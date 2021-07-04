pub mod execute;
pub mod query;
pub mod sync;

use serde::{Deserialize, Serialize};

use strum::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter, strum::Display)]
#[repr(u8)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceStatus {
    /// Confirm that the command succeeded.
    Success,

    /// Target device is in offline state or unreachable.
    Offline,

    /// There is an issue or alert associated with a query.
    /// The query could succeed or fail.
    /// This status type is typically set when you want to send additional information about another connected device.
    Exceptions,

    /// Target device is unable to perform the command.
    Error,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentRequest {
    pub request_id: String,
    pub inputs: Vec<IntentRequestInput>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "intent", content = "payload")]
pub enum IntentRequestInput {
    #[serde(rename = "actions.devices.SYNC")]
    Sync(sync::request::Payload),

    #[serde(rename = "actions.devices.QUERY")]
    Query(query::request::Payload),

    #[serde(rename = "actions.devices.EXECUTE")]
    Execute(execute::request::Payload),

    #[serde(rename = "actions.devices.DISCONNECT")]
    Disconnect,
}

pub type IntentResponse = Result<IntentResponseBody, IntentResponseError>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum IntentResponseBody {
    Sync {
        request_id: String,
        payload: sync::response::Payload,
    },
    Query {
        request_id: String,
        payload: query::response::Payload,
    },
    Execute {
        request_id: String,
        payload: execute::response::Payload,
    },
    Disconnect,
}

use crate::{lighthouse, token};

#[derive(Debug, Clone, Deserialize, Serialize, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error_description",
    rename_all = "snake_case"
)]
pub enum IntentResponseError {
    #[error("internal error: {0}")]
    InternalError(#[from] crate::InternalServerError),

    #[error("token error: {0}")]
    TokenError(#[from] token::Error),

    #[error("no device permission")]
    NoDevicePermission,

    #[error("error with device communication: {0}")]
    DeviceCommunicationError(#[from] lighthouse::DeviceCommunicationError),
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for IntentResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TokenError(_) => StatusCode::UNAUTHORIZED,
            Self::NoDevicePermission => StatusCode::UNAUTHORIZED,
            Self::DeviceCommunicationError(_) => StatusCode::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        crate::json_error_response(self.status_code(), self)
    }
}
