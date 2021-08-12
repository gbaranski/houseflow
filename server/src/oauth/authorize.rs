use super::{verify_redirect_uri, AuthorizationRequestQuery, Error};
use crate::State;
use axum::extract::{Extension, Query};

const AUTHORIZE_PAGE: &str = include_str!("authorize.html");

#[tracing::instrument(name = "Authorization", skip(state), err)]
pub async fn handle(
    Extension(state): Extension<State>,
    Query(request): Query<AuthorizationRequestQuery>,
) -> Result<http::Response<axum::body::Body>, Error> {
    let google_config = state.config.google.as_ref().unwrap();
    if *request.client_id != *google_config.client_id {
        return Err(Error::InvalidClient(Some(String::from(
            "invalid client id",
        ))));
    }
    verify_redirect_uri(&request.redirect_uri, &google_config.project_id)
        .map_err(|err| Error::InvalidRequest(Some(err.to_string())))?;

    let response = http::Response::builder()
        .header("Content-Type", "text/html")
        .body(axum::body::Body::from(AUTHORIZE_PAGE))
        .unwrap();

    Ok(response)
}
