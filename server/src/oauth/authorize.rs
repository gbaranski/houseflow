use super::verify_redirect_uri;
use super::AuthorizationRequestQuery;
use crate::State;
use axum::extract::Extension;
use axum::extract::Query;
use houseflow_types::errors::OAuthError;
use houseflow_types::errors::ServerError;

const AUTHORIZE_PAGE: &str = include_str!("authorize.html");

#[tracing::instrument(name = "Authorization", skip(state), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    Query(request): Query<AuthorizationRequestQuery>,
) -> Result<http::Response<axum::body::Body>, ServerError> {
    let google_config = state.config.google.as_ref().unwrap();
    if *request.client_id != *google_config.client_id {
        return Err(OAuthError::InvalidClient(Some(String::from("invalid client id"))).into());
    }
    verify_redirect_uri(&request.redirect_uri, &google_config.project_id)
        .map_err(|err| OAuthError::InvalidRequest(Some(err.to_string())))?;

    let response = http::Response::builder()
        .header("Content-Type", "text/html")
        .body(axum::body::Body::from(AUTHORIZE_PAGE))
        .unwrap();

    Ok(response)
}
