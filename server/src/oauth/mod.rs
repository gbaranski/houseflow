pub mod authorize;
pub mod login;
pub mod token;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationResponseType {
    Code,
}

#[derive(Deserialize)]
pub struct AuthorizationRequestQuery {
    pub client_id: String,
    pub redirect_uri: Url,
    pub state: String,

    // TODO: remove dead_code permission
    #[allow(dead_code)]
    pub scope: Option<String>,

    #[allow(dead_code)]
    pub response_type: AuthorizationResponseType,

    #[allow(dead_code)]
    #[serde(default = "default_user_locale")]
    pub user_locale: String,
}

fn default_user_locale() -> String {
    String::from("en_US")
}

#[derive(Serialize, Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum InvalidRedirectURIError {
    #[error("invalid scheme: {0}")]
    InvalidScheme(String),

    #[error("invalid host")]
    InvalidHost,

    #[error("invalid path")]
    InvalidPath,

    #[error("invalid project id")]
    InvalidProjectID,
}

const GOOGLE_OAUTH_REDIRECT_URL: &str = "oauth-redirect.googleusercontent.com";
const GOOGLE_SANDBOX_OAUTH_REDIRECT_URL: &str = "oauth-redirect-sandbox.googleusercontent.com";

fn verify_redirect_uri(
    redirect_uri: &Url,
    project_id: &str,
) -> Result<(), InvalidRedirectURIError> {
    let scheme = redirect_uri.scheme();
    let host = match redirect_uri.host() {
        Some(url::Host::Domain(s)) => Ok(s),
        _ => Err(InvalidRedirectURIError::InvalidHost),
    }?;

    let mut segments = redirect_uri
        .path_segments()
        .ok_or(InvalidRedirectURIError::InvalidPath)?;

    let first_segment = segments
        .next()
        .ok_or(InvalidRedirectURIError::InvalidPath)?;
    let second_segment = segments
        .next()
        .ok_or(InvalidRedirectURIError::InvalidPath)?;

    if scheme != "https" {
        Err(InvalidRedirectURIError::InvalidScheme(scheme.to_string()))
    } else if host != GOOGLE_OAUTH_REDIRECT_URL && host != GOOGLE_SANDBOX_OAUTH_REDIRECT_URL {
        Err(InvalidRedirectURIError::InvalidHost)
    } else if first_segment != "r" {
        Err(InvalidRedirectURIError::InvalidPath)
    } else if second_segment != project_id {
        Err(InvalidRedirectURIError::InvalidProjectID)
    } else {
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("internal error: {0}")]
    InternalError(#[from] houseflow_types::errors::InternalError),

    /// The request is missing a parameter so the server can’t proceed with the request.
    /// This may also be returned if the request includes an unsupported parameter or repeats a parameter.
    #[error("invalid request, description: {0:?}")]
    InvalidRequest(Option<String>),

    /// Client authentication failed, such as if the request contains an invalid client ID or secret.
    /// Send an HTTP 401 response in this case.
    #[error("invalid clientid or secret, description: {0:?}")]
    InvalidClient(Option<String>),

    /// The authorization code (or user’s password for the password grant type) is invalid or expired.
    /// This is also the error you would return if the redirect URL given in the authorization grant does not match the URL provided in this access token request.
    #[error("invalid grant, description: {0:?}")]
    InvalidGrant(Option<String>),

    /// For access token requests that include a scope (password or client_credentials grants), this error indicates an invalid scope value in the request.
    #[error("invalid scope, description: {0:?}")]
    InvalidScope(Option<String>),

    /// This client is not authorized to use the requested grant type.
    /// For example, if you restrict which applications can use the Implicit grant, you would return this error for the other apps.
    #[error("unauthorized client, description: {0:?}")]
    UnauthorizedClient(Option<String>),

    /// If a grant type is requested that the authorization server doesn’t recognize, use this code.
    /// Note that unknown grant types also use this specific error code rather than using the invalid_request above.
    #[error("unsupported grant type, description: {0:?}")]
    UnsupportedGrantType(Option<String>),
}

impl axum::response::IntoResponse for Error {
    type Body = http_body::Full<hyper::body::Bytes>;

    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> http::Response<Self::Body> {
        use http::StatusCode;
        let status = match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };
        let mut response = axum::Json(self).into_response();
        *response.status_mut() = status;

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    mod redirect_uri {
        use super::*;
        const PROJECT_ID: &str = "some-project-id";

        #[test]
        fn valid() {
            assert!(verify_redirect_uri(
                &Url::parse(&format!(
                    "https://{}/r/{}",
                    GOOGLE_OAUTH_REDIRECT_URL, PROJECT_ID
                ))
                .unwrap(),
                PROJECT_ID,
            )
            .is_ok());

            assert!(verify_redirect_uri(
                &Url::parse(&format!(
                    "https://{}/r/{}",
                    GOOGLE_SANDBOX_OAUTH_REDIRECT_URL, PROJECT_ID
                ))
                .unwrap(),
                PROJECT_ID,
            )
            .is_ok());
        }

        #[test]
        fn invalid_project_id() {
            assert!(verify_redirect_uri(
                &Url::parse(&format!(
                    "https://{}/r/{}",
                    GOOGLE_SANDBOX_OAUTH_REDIRECT_URL, "invalid-project-id"
                ))
                .unwrap(),
                PROJECT_ID,
            )
            .is_err());
        }

        #[test]
        fn no_tls() {
            assert!(verify_redirect_uri(
                &Url::parse(&format!(
                    "http://{}/r/{}",
                    GOOGLE_SANDBOX_OAUTH_REDIRECT_URL, PROJECT_ID
                ))
                .unwrap(),
                PROJECT_ID,
            )
            .is_err());
        }
    }
}
