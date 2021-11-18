use super::verify_redirect_uri;
use super::AuthorizationRequestQuery;
use crate::State;
use askama::Template;
use axum::extract::Extension;
use axum::extract::Query;
use axum::response::Html;
use houseflow_types::errors::InternalError;
use houseflow_types::errors::OAuthError;
use houseflow_types::errors::ServerError;
use http::HeaderMap;
use url::Url;

#[derive(Template)]
#[template(path = "authorize.html")]
struct AuthorizeTemplate {
    client_id: String,
    redirect_uri: Url,
    state: String,
    base_url: Url,
    google_login_client_id: Option<String>,
}

#[tracing::instrument(name = "Authorization", skip(state), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    Query(request): Query<AuthorizationRequestQuery>,
    headers: HeaderMap,
) -> Result<Html<String>, ServerError> {
    let google_config = state
        .config
        .google
        .as_ref()
        .ok_or_else(|| InternalError::Other("Google Home API not configured".to_string()))?;
    if *request.client_id != *google_config.client_id {
        return Err(OAuthError::InvalidClient(Some(String::from("invalid client id"))).into());
    }
    verify_redirect_uri(&request.redirect_uri, &google_config.project_id)
        .map_err(|err| OAuthError::InvalidRequest(Some(err.to_string())))?;

    let template = AuthorizeTemplate {
        client_id: request.client_id.to_owned(),
        redirect_uri: request.redirect_uri.to_owned(),
        state: request.state.to_owned(),
        base_url: state.config.get_base_url(),
        google_login_client_id: state
            .config
            .logins
            .google
            .as_ref()
            .map(|c| c.client_id.to_owned()),
    };
    Ok(Html(template.render()?))
}
