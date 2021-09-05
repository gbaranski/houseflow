use super::verify_redirect_uri;
use super::AuthorizationRequestQuery;
use super::Error;
use crate::State;
use axum::extract::Extension;
use axum::extract::Form;
use axum::extract::Query;
use chrono::Duration;
use chrono::Utc;
use houseflow_types::auth::login::Request;
use houseflow_types::code::VerificationCode;
use houseflow_types::errors::InternalError;
use houseflow_types::token::AuthorizationCode;
use houseflow_types::token::AuthorizationCodePayload;

const VERIFICATION_CODE_DURATION: std::time::Duration = std::time::Duration::from_secs(60 * 30);

#[tracing::instrument(
    name = "Login",
    skip(state, request, query),
    fields(
        email = %request.email,
    ),
    err,
)]
pub async fn handle(
    Extension(state): Extension<State>,
    Form(request): Form<Request>,
    Query(query): Query<AuthorizationRequestQuery>,
) -> Result<http::Response<axum::body::Body>, Error> {
    validator::Validate::validate(&request)
        .map_err(|err| Error::InvalidRequest(Some(err.to_string())))?;

    let google_config = state.config.google.as_ref().unwrap();
    if *query.client_id != *google_config.client_id {
        return Err(Error::InvalidClient(Some(String::from(
            "invalid client id",
        ))));
    }
    verify_redirect_uri(&query.redirect_uri, &google_config.project_id)
        .map_err(|err| Error::InvalidRequest(Some(err.to_string())))?;

    let user = state
        .config
        .get_user_by_email(&request.email)
        .ok_or_else(|| Error::InvalidGrant(Some(String::from("user not found"))))?;

    let response = match request.verification_code {
        Some(verification_code) => {
            let user_id = state
                .clerk
                .get(&verification_code)
                .await
                .map_err(|err| Error::Internal(InternalError::Clerk(err.to_string())))?
                .ok_or_else(|| {
                    Error::InvalidRequest(Some(
                        "verification code is not known by clerk".to_string(),
                    ))
                })?;
            if user_id != user.id {
                return Err(Error::InvalidRequest(Some(
                    "verification code user-id doesn't match".to_string(),
                )));
            }

            let authorization_code_payload = AuthorizationCodePayload {
                sub: user.id,
                exp: Utc::now() + Duration::minutes(10),
            };
            let authorization_code = AuthorizationCode::new(
                state.config.secrets.authorization_code_key.as_bytes(),
                authorization_code_payload,
            );
            let mut redirect_uri = query.redirect_uri;
            redirect_uri.set_query(Some(&format!(
                "code={}&state={}",
                authorization_code, query.state
            )));

            tracing::info!(%user_id, "Authorization code granted");

            http::Response::builder()
                .status(http::StatusCode::SEE_OTHER)
                .header("Location", redirect_uri.to_string())
                .body(axum::body::Body::empty())
                .unwrap()
        }
        None => {
            let verification_code: VerificationCode = rand::random();
            state
                .clerk
                .add(
                    verification_code.clone(),
                    user.id.clone(),
                    Utc::now() + chrono::Duration::from_std(VERIFICATION_CODE_DURATION).unwrap(),
                )
                .await
                .map_err(|err| Error::Internal(InternalError::Clerk(err.to_string())))?;
            state
                .mailer
                .send_verification_code(&user.email, &verification_code)
                .await
                .map_err(|err| Error::Internal(InternalError::Mailer(err.to_string())))?;

            http::Response::builder()
                .status(http::StatusCode::OK)
                .body(axum::body::Body::from(format!("Verification code sent to {}. Please copy the code and re-send the form with the code.", user.email)))
                .unwrap()
        }
    };
    Ok(response)
}
