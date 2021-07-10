use super::{verify_redirect_uri, AuthorizationRequestQuery, AuthorizationResponseError};
use actix_web::{
    get,
    web::{self, Data},
    HttpResponse,
};

const AUTHORIZE_PAGE: &str = include_str!("authorize.html");

#[get("/authorize")]
pub async fn on_authorize(
    request: web::Query<AuthorizationRequestQuery>,
    server_config: Data<houseflow_config::server::Config>,
) -> Result<HttpResponse, AuthorizationResponseError> {
    let google_config = server_config.google.as_ref().unwrap();
    if *request.client_id != *google_config.client_id {
        return Err(AuthorizationResponseError::InvalidClientID);
    }
    verify_redirect_uri(&request.redirect_uri, &google_config.project_id)?;

    let response = HttpResponse::build(actix_web::http::StatusCode::OK)
        .content_type("text/html")
        .body(AUTHORIZE_PAGE);

    Ok(response)
}
