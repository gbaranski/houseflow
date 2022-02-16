use crate::extensions;
use crate::mailer::MailerExt;
use axum::Json;
use chrono::Duration;
use chrono::Utc;
use houseflow_types::auth::login::Request;
use houseflow_types::auth::login::Response;
use houseflow_types::code::VerificationCode;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::ServerError;
use houseflow_types::token::AccessToken;
use houseflow_types::token::AccessTokenClaims;
use houseflow_types::token::RefreshToken;
use houseflow_types::token::RefreshTokenClaims;
use tracing::Level;

const VERIFICATION_CODE_DURATION: std::time::Duration = std::time::Duration::from_secs(60 * 30);
const VERIFICATION_CODE_LIMIT: usize = 3;

#[tracing::instrument(
    name = "Login",
    skip(config, clerk, mailer, request),
    fields(
        email = %request.email,
    ),
    err,
)]
pub async fn handle(
    config: extensions::Config,
    clerk: extensions::Clerk,
    mailer: extensions::MasterMailer,
    Json(request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    validator::Validate::validate(&request)?;
    let user = config
        .get()
        .get_user_by_email(&request.email)
        .ok_or(AuthError::UserNotFound)?
        .to_owned();

    let response = match request.verification_code {
        Some(verification_code) => {
            let user_id = clerk.get(&verification_code).await?.ok_or_else(|| {
                AuthError::InvalidVerificationCode("code is not known by clerk".to_string())
            })?;
            if user_id != user.id {
                return Err(AuthError::InvalidVerificationCode(
                    "user-id doesn't match".to_string(),
                )
                .into());
            }
            let refresh_token = RefreshToken::new(
                config.get().secrets.refresh_key.as_bytes(),
                RefreshTokenClaims {
                    sub: user.id,
                    exp: Some(Utc::now() + Duration::days(7)),
                },
            )?;
            let access_token = AccessToken::new(
                config.get().secrets.access_key.as_bytes(),
                AccessTokenClaims {
                    sub: user.id,
                    exp: Utc::now() + Duration::minutes(10),
                },
            )?;
            tracing::event!(Level::INFO, user_id = %user.id, "Logged in");
            Response::LoggedIn {
                access_token: access_token.encode(),
                refresh_token: refresh_token.encode(),
            }
        }
        None => {
            if clerk.count_verification_codes_for_user(&user.id)? > VERIFICATION_CODE_LIMIT {
                return Err(ServerError::TooManyRequests);
            }
            let verification_code: VerificationCode = rand::random();
            clerk
                .add(
                    verification_code.clone(),
                    user.id,
                    Utc::now() + chrono::Duration::from_std(VERIFICATION_CODE_DURATION).unwrap(),
                )
                .await?;
            mailer
                .send_verification_code(
                    String::from("Your Houseflow account: Access from a new computer"),
                    user.email.to_owned(),
                    verification_code,
                )
                .await;
            Response::VerificationCodeSent
        }
    };

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::Request;
    use super::Response;
    use super::VERIFICATION_CODE_DURATION;
    use crate::test_utils::*;
    use axum::Json;
    use chrono::Utc;
    use houseflow_types::code::VerificationCode;
    use houseflow_types::errors::AuthError;
    use houseflow_types::errors::ServerError;
    use houseflow_types::token::AccessToken;
    use houseflow_types::token::RefreshToken;
    use houseflow_types::user;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn valid() {
        let user = get_user();
        let (mailer_tx, mut mailer_rx) = mpsc::unbounded_channel();
        let config = get_config(GetConfig {
            users: vec![user.clone()],
            ..Default::default()
        })
        .await;
        let clerk = get_clerk(GetClerk::default()).await;
        let mailer = get_master_mailer(GetMasterMailer {
            tx: Some(mailer_tx),
        })
        .await;
        let Json(response) = super::handle(
            config.clone(),
            clerk.clone(),
            mailer.clone(),
            Json(Request {
                email: user.email.clone(),
                verification_code: None,
            }),
        )
        .await
        .unwrap();
        assert_eq!(response, Response::VerificationCodeSent);
        let (address, verification_code) = mailer_rx.recv().await.unwrap();
        assert_eq!(address, user.email);
        let Json(response) = super::handle(
            config.clone(),
            clerk,
            mailer,
            Json(Request {
                email: user.email.clone(),
                verification_code: Some(verification_code),
            }),
        )
        .await
        .unwrap();
        let (at, rt) = match response {
            Response::LoggedIn {
                access_token,
                refresh_token,
            } => (access_token, refresh_token),
            _ => panic!("expected Response::LoggedIn"),
        };
        let (at, rt) = (
            AccessToken::decode(config.get().secrets.access_key.as_bytes(), &at).unwrap(),
            RefreshToken::decode(config.get().secrets.refresh_key.as_bytes(), &rt).unwrap(),
        );
        assert_eq!(at.claims.sub, rt.claims.sub);
    }

    #[tokio::test]
    async fn verification_code_unknown_by_clerk() {
        let user = get_user();
        let config = get_config(GetConfig {
            users: vec![user.clone()],
            ..Default::default()
        })
        .await;
        let clerk = get_clerk(GetClerk::default()).await;
        let mailer = get_master_mailer(GetMasterMailer::default()).await;
        let verification_code: VerificationCode = rand::random();
        let response = super::handle(
            config,
            clerk,
            mailer,
            Json(Request {
                email: user.email,
                verification_code: Some(verification_code),
            }),
        )
        .await
        .unwrap_err();

        assert!(matches!(response, ServerError::AuthError(_)))
    }

    #[tokio::test]
    async fn verification_code_invalid_user_id() {
        let user = get_user();
        let config = get_config(GetConfig {
            users: vec![user.clone()],
            ..Default::default()
        })
        .await;
        let clerk = get_clerk(GetClerk::default()).await;
        let mailer = get_master_mailer(GetMasterMailer::default()).await;
        let verification_code: VerificationCode = rand::random();
        clerk
            .add(
                verification_code.clone(),
                user::ID::new_v4(),
                Utc::now() + chrono::Duration::from_std(VERIFICATION_CODE_DURATION).unwrap(),
            )
            .await
            .unwrap();

        let response = super::handle(
            config,
            clerk,
            mailer,
            Json(Request {
                email: user.email,
                verification_code: Some(verification_code),
            }),
        )
        .await
        .unwrap_err();

        assert!(matches!(response, ServerError::AuthError(_)))
    }

    #[tokio::test]
    async fn not_existing_user() {
        let user = get_user();
        let config = get_config(GetConfig::default()).await;
        let clerk = get_clerk(GetClerk::default()).await;
        let mailer = get_master_mailer(GetMasterMailer::default()).await;
        let response = super::handle(
            config,
            clerk,
            mailer,
            Json(Request {
                email: user.email,
                verification_code: None,
            }),
        )
        .await
        .unwrap_err();

        assert_eq!(response, ServerError::AuthError(AuthError::UserNotFound));
    }
}
