pub mod authorize;
pub mod google_login;
pub mod login;
pub mod token;

use chrono::Duration;
use chrono::Utc;
use houseflow_config::server::Google;
use houseflow_config::server::Secrets;
use houseflow_types::errors::OAuthError;
use houseflow_types::errors::TokenError;
use houseflow_types::token::AuthorizationCode;
use houseflow_types::token::AuthorizationCodePayload;
use houseflow_types::user::ID as UserID;
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

fn verify_oauth_query(
    query: &AuthorizationRequestQuery,
    google_config: &Google,
) -> Result<(), OAuthError> {
    if *query.client_id != *google_config.client_id {
        return Err(OAuthError::InvalidClient(Some(
            "invalid client id".to_string(),
        )));
    }
    verify_redirect_uri(&query.redirect_uri, &google_config.project_id)
        .map_err(|err| OAuthError::InvalidRequest(Some(err.to_string())))?;
    Ok(())
}

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

/// The given user has successfully authenticated, so grant them an OAuth authentication code by
/// redirecting to the redirect_uri.
fn grant_authorization_code(
    query: AuthorizationRequestQuery,
    user_id: UserID,
    secrets: &Secrets,
) -> Result<http::Response<axum::body::Body>, TokenError> {
    let authorization_code_payload = AuthorizationCodePayload {
        sub: user_id,
        exp: Utc::now() + Duration::minutes(10),
    };
    let authorization_code = AuthorizationCode::new(
        secrets.authorization_code_key.as_bytes(),
        authorization_code_payload,
    )?;
    let mut redirect_uri = query.redirect_uri;
    redirect_uri.set_query(Some(&format!(
        "code={}&state={}",
        authorization_code, query.state
    )));

    tracing::info!(%user_id, "Authorization code granted");

    Ok(http::Response::builder()
        .status(http::StatusCode::SEE_OTHER)
        .header("Location", redirect_uri.to_string())
        .body(axum::body::Body::empty())
        .unwrap())
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
