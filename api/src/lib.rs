#[cfg(feature = "admin")]
mod admin;

#[cfg(feature = "auth")]
mod auth;

#[cfg(feature = "auth")]
mod fulfillment;

#[cfg(feature = "auth")]
pub use crate::fulfillment::FulfillmentError;

#[cfg(any(feature = "auth", feature = "fulfillment", feature = "admin"))]
use url::Url;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),
}

#[derive(Debug, Clone)]
pub struct HouseflowAPI {
    #[cfg(feature = "auth")]
    auth_url: Url,

    #[cfg(feature = "fulfillment")]
    fulfillment_url: Url,

    #[cfg(feature = "admin")]
    admin_url: Url,
}

impl HouseflowAPI {
    pub fn new(#[allow(unused_variables)] server_address: std::net::SocketAddr) -> Self {
        Self {
            #[cfg(feature = "auth")]
            auth_url: Url::parse(&format!("http://{}/auth/", server_address)).unwrap(),

            #[cfg(feature = "fulfillment")]
            fulfillment_url: Url::parse(&format!(
                "http://{}/fulfillment/internal/",
                server_address
            ))
            .unwrap(),

            #[cfg(feature = "admin")]
            admin_url: Url::parse(&format!("http://{}/admin/", server_address)).unwrap(),
        }
    }
}
