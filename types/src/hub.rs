use serde::Deserialize;
use crate::accessory;
use serde::Serialize;
use uuid::Uuid;

pub type ID = Uuid;
pub type Password = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Hub {
    pub id: ID,
    pub name: String,
    pub password_hash: Option<String>,
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
#[serde(
    tag = "error",
    content = "error-description",
    rename_all = "kebab-case"
)]
pub enum Error {
    #[error("accessory: {0}",)]
    AccessoryError(#[from] accessory::Error),
}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;

        let status = match &self {
            Self::AccessoryError(err) => match err {
                accessory::Error::CharacteristicReadOnly => StatusCode::BAD_REQUEST,
                accessory::Error::CharacteristicWriteOnly => StatusCode::BAD_REQUEST,
                accessory::Error::CharacteristicNotSupported => StatusCode::BAD_REQUEST,
                accessory::Error::ServiceNotSupported => StatusCode::BAD_REQUEST,
                accessory::Error::NotConnected => StatusCode::SERVICE_UNAVAILABLE,
            },
        };
        let mut response = axum::Json(self).into_response();
        *response.status_mut() = status;

        response
    }
}
