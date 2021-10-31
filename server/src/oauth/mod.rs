pub mod authorize;
pub mod google_login;
pub mod login;
pub mod token;

use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationResponseType {
    Code,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, thiserror::Error)]
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
