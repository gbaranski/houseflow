#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "hub")]
pub mod hub;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "http_api")]
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[cfg(feature = "http_api")]
    #[error("invalid response body, code: `{status_code}`, error: `{error}`, body: `{body}`")]
    InvalidResponseBody {
        error: Box<dyn std::error::Error + Send + Sync>,
        status_code: reqwest::StatusCode,
        body: String,
    },
}
