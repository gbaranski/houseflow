use crate::extensions;
use crate::extractors::RefreshToken;
use axum::Json;
use chrono::Duration;
use chrono::Utc;
use houseflow_types::auth::token::Request;
use houseflow_types::auth::token::Response;
use houseflow_types::errors::ServerError;
use houseflow_types::token::AccessToken;
use houseflow_types::token::AccessTokenClaims;
use tracing::Level;

#[tracing::instrument(name = "Refresh token", skip(config, _request), err)]
pub async fn handle(
    config: extensions::Config,
    RefreshToken(refresh_token): RefreshToken,
    Json(_request): Json<Request>,
) -> Result<Json<Response>, ServerError> {
    let access_token_payload = AccessTokenClaims {
        sub: refresh_token.claims.sub,
        exp: Utc::now() + Duration::minutes(10),
    };
    let access_token = AccessToken::new(
        config.get().secrets.access_key.as_bytes(),
        access_token_payload,
    )?;

    tracing::event!(Level::INFO, user_id = %refresh_token.claims.sub, "Refreshed token");

    Ok(Json(Response {
        refresh_token: None,
        access_token: access_token.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;
    use axum::Json;
    use houseflow_types::token::RefreshToken;
    use houseflow_types::token::RefreshTokenClaims;

    #[tokio::test]
    async fn valid() {
        let user = get_user();
        let config = get_config(GetConfig {
            users: vec![user.clone()],
            ..Default::default()
        })
        .await;
        let refresh_token = RefreshToken::new(
            config.get().secrets.refresh_key.as_bytes(),
            RefreshTokenClaims {
                sub: user.id,
                exp: None,
            },
        )
        .unwrap();
        let Json(response) = super::handle(
            config.clone(),
            crate::extractors::RefreshToken(refresh_token.clone()),
            Json(super::Request {}),
        )
        .await
        .unwrap();
        let access_token = houseflow_types::token::AccessToken::decode(
            config.get().secrets.access_key.as_bytes(),
            &response.access_token,
        )
        .unwrap();
        assert_eq!(access_token.claims.sub, refresh_token.sub);
    }
}
