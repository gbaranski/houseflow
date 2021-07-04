mod common;
mod device;
mod user;

#[cfg(feature = "admin")]
pub mod admin;

#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "fulfillment")]
pub mod fulfillment;

#[cfg(feature = "lighthouse")]
pub mod lighthouse;

#[cfg(feature = "token")]
pub mod token;

pub use common::*;
pub use device::*;
pub use user::*;

#[cfg(feature = "actix")]
pub(crate) fn json_error_response(
    status_code: actix_web::http::StatusCode,
    err: &impl serde::ser::Serialize,
) -> actix_web::HttpResponse {
    let json = actix_web::web::Json(err);
    actix_web::HttpResponse::build(status_code).json(json)
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum InternalServerError {
    #[error("database error: {0}")]
    DatabaseError(String),

    #[error("token store error: {0}")]
    TokenStoreError(String),

    #[error("other error: {0}")]
    Other(String),
}

#[cfg(feature = "validator")]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationError(validator::ValidationError);

#[cfg(feature = "validator")]
impl From<validator::ValidationErrors> for ValidationError {
    fn from(errors: validator::ValidationErrors) -> Self {
        Self(
            errors
                .field_errors()
                .iter()
                .next()
                .unwrap()
                .1
                .first()
                .unwrap()
                .clone(),
        )
    }
}

#[cfg(feature = "validator")]
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "validator")]
impl std::error::Error for ValidationError {}
