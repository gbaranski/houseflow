use super::grant_authorization_code;
use super::verify_oauth_query;
use super::AuthorizationRequestQuery;
use crate::State;
use axum::extract::Extension;
use axum::extract::Form;
use axum::extract::Query;
use chrono::Utc;
use houseflow_types::auth::login::Request;
use houseflow_types::code::VerificationCode;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::InternalError;
use houseflow_types::errors::OAuthError;
use houseflow_types::errors::ServerError;

const VERIFICATION_CODE_DURATION: std::time::Duration = std::time::Duration::from_secs(60 * 30);
const VERIFICATION_CODE_LIMIT: usize = 3;

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
) -> Result<http::Response<axum::body::Body>, ServerError> {
    validator::Validate::validate(&request)?;

    let google_config = state
        .config
        .google
        .as_ref()
        .ok_or_else(|| InternalError::Other("Google Home API not configured".to_string()))?;
    verify_oauth_query(&query, google_config)?;

    let user = state
        .config
        .get_user_by_email(&request.email)
        .ok_or_else(|| OAuthError::InvalidGrant(Some(String::from("user not found"))))?;

    let response = match request.verification_code {
        Some(verification_code) => {
            let user_id = state.clerk.get(&verification_code).await?.ok_or_else(|| {
                AuthError::InvalidVerificationCode(
                    "verification code is not known by clerk".to_string(),
                )
            })?;
            if user_id != user.id {
                return Err(OAuthError::InvalidRequest(Some(
                    "verification code user-id doesn't match".to_string(),
                ))
                .into());
            }

            grant_authorization_code(query, user.id, &state.config.secrets)?
        }
        None => {
            if state.clerk.count_verification_codes_for_user(&user.id)? > VERIFICATION_CODE_LIMIT {
                return Err(ServerError::TooManyRequests);
            }
            let verification_code: VerificationCode = rand::random();
            state
                .clerk
                .add(
                    verification_code.clone(),
                    user.id,
                    Utc::now() + chrono::Duration::from_std(VERIFICATION_CODE_DURATION).unwrap(),
                )
                .await?;
            state
                .mailer
                .send_verification_code(&user.email, &verification_code)
                .await?;

            http::Response::builder()
                    .status(http::StatusCode::OK)
                    .body(axum::body::Body::from(format!("Verification code sent to {}. Please copy the code and re-send the form with the code.", user.email)))
                    .unwrap()
        }
    };
    Ok(response)
}
