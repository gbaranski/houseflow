#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::Client as AsyncClient;

#[cfg(feature = "sync")]
mod sync;
#[cfg(feature = "sync")]
pub use sync::Client;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[error("invalid response body, code: `{status_code}`, error: `{error}`, body: `{body}`")]
    InvalidResponseBody {
        error: Box<dyn std::error::Error + Send + Sync>,
        status_code: reqwest::StatusCode,
        body: String,
    },
}
