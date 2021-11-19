use super::grant_authorization_code;
use super::verify_oauth_query;
use super::AuthorizationRequestQuery;
use crate::State;
use axum::extract::Extension;
use axum::extract::Form;
use axum::extract::Query;
use axum::extract::TypedHeader;
use headers::Cookie;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::InternalError;
use houseflow_types::errors::OAuthError;
use houseflow_types::errors::ServerError;
use jsonwebtoken_google::Parser;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    credential: String,
    g_csrf_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenClaims {
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub picture: String,
    pub aud: String,
    pub iss: String,
    pub exp: u64,
}

#[tracing::instrument(name = "GoogleLogin", skip(state, request, cookies), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    Form(request): Form<Request>,
    Query(query): Query<AuthorizationRequestQuery>,
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Result<http::Response<axum::body::Body>, ServerError> {
    if request.g_csrf_token != cookies.get("g_csrf_token").unwrap_or("") {
        return Err(AuthError::InvalidCsrfToken.into());
    }
    let google_config = state
        .config
        .google
        .as_ref()
        .ok_or_else(|| InternalError::Other("Google Home API not configured".to_string()))?;
    let google_login_config = state
        .config
        .logins
        .google
        .as_ref()
        .ok_or_else(|| InternalError::Other("Google login not configured".to_string()))?;
    verify_oauth_query(&query, google_config)?;

    // Validate JWT and parse claims.
    // See https://developers.google.com/identity/gsi/web/guides/verify-google-id-token
    let parser = Parser::new(&google_login_config.client_id);
    let claims = parser
        .parse::<TokenClaims>(&request.credential)
        .await
        .map_err(|e| AuthError::InvalidGoogleJwt(e.to_string()))?;

    // User has successfully authenticated with Google, see if they exist in our config.
    let user = state
        .config
        .get_user_by_email(&claims.email)
        .ok_or_else(|| OAuthError::InvalidGrant(Some(String::from("user not found"))))?;

    Ok(grant_authorization_code(
        query,
        user.id,
        &state.config.secrets,
    )?)
}
