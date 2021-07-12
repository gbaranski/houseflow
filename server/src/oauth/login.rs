use actix_web::{
    web::{Data, Form, Query},
    HttpResponse,
};
use chrono::{Duration, Utc};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    auth::login::Request,
    token::{AuthorizationCode, AuthorizationCodePayload},
};

use super::{verify_redirect_uri, AuthorizationRequestQuery, AuthorizationResponseError};

fn verify_password(hash: &str, password: &str) -> Result<(), ResponseError> {
    match argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
        true => Ok(()),
        false => Err(ResponseError::InvalidPassword),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ResponseError {
    #[error("internal error: {0}")]
    InternalError(#[from] houseflow_types::InternalServerError),

    #[error("validation error: {0}")]
    ValidationError(#[from] houseflow_types::ValidationError),

    #[error("{0}")]
    Authorize(#[from] AuthorizationResponseError),

    #[error("invalid password")]
    InvalidPassword,

    #[error("user not found")]
    UserNotFound,
}

impl actix_web::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InvalidPassword => StatusCode::UNAUTHORIZED,
            Self::UserNotFound => StatusCode::UNAUTHORIZED,
            Self::Authorize(err) => err.status_code(),
        }
    }
}

pub async fn on_login(
    Form(request): Form<Request>,
    Query(query): Query<AuthorizationRequestQuery>,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<HttpResponse, ResponseError> {
    validator::Validate::validate(&request).map_err(houseflow_types::ValidationError::from)?;
    let user = db
        .get_user_by_email(&request.email)
        .map_err(houseflow_db::Error::into_internal_server_error)?
        .ok_or(ResponseError::UserNotFound)?;

    verify_password(&user.password_hash, &request.password)?;
    let google_config = config.google.as_ref().unwrap();
    if *query.client_id != *google_config.client_id {
        return Err(ResponseError::Authorize(
            AuthorizationResponseError::InvalidClientID,
        ));
    }
    verify_redirect_uri(&query.redirect_uri, &google_config.project_id)
        .map_err(AuthorizationResponseError::InvalidRedirectURI)?;
    let authorization_code_payload = AuthorizationCodePayload {
        sub: user.id,
        exp: Utc::now() + Duration::minutes(10),
    };
    let authorization_code = AuthorizationCode::new(
        config.secrets.authorization_code_key.as_bytes(),
        authorization_code_payload,
    );
    let mut redirect_uri = query.redirect_uri;
    redirect_uri.set_query(Some(&format!(
        "code={}&state={}",
        authorization_code, query.state
    )));

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", redirect_uri.to_string()))
        .body(""))
}
