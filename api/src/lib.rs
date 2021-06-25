#[cfg(feature = "auth")]
mod auth;

#[cfg(feature = "auth")]
mod fulfillment;

#[cfg(feature = "auth")]
pub use crate::fulfillment::FulfillmentError;

#[cfg(feature = "auth")]
pub use crate::auth::AuthError;

#[cfg(any(feature = "auth", feature = "fulfillment"))]
use url::Url;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[cfg(feature = "auth")]
    #[error("auth error: {0}")]
    AuthError(#[from] AuthError),
}

#[derive(Debug, Clone)]
pub struct HouseflowAPI {
    #[cfg(feature = "auth")]
    auth_url: Url,

    #[cfg(feature = "fulfillment")]
    fulfillment_url: Url,
}

impl HouseflowAPI {
    pub fn new(server_address: std::net::SocketAddr) -> Self {
        Self {
            #[cfg(feature = "auth")]
            auth_url: Url::parse(&format!("http://{}/auth/", server_address)).unwrap(),

            #[cfg(feature = "fulfillment")]
            fulfillment_url: Url::parse(&format!(
                "http://{}/fulfillment/internal/",
                server_address
            ))
            .unwrap(),
        }
    }
}
