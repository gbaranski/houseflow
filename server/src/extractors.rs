use crate::State;
use async_trait::async_trait;
use axum::body::Body;
use houseflow_config::server::Secrets;
use houseflow_types::{
    errors::{AuthError, ServerError, TokenError},
    token::{AccessTokenPayload, RefreshTokenPayload, Token},
};

use serde::{de, ser};

pub struct RefreshToken(pub Token<RefreshTokenPayload>);
pub struct AccessToken(pub Token<AccessTokenPayload>);

async fn from_request<P>(
    req: &mut axum::extract::RequestParts<Body>,
    get_key_fn: impl FnOnce(&Secrets) -> &str,
) -> Result<Token<P>, AuthError>
where
    P: ser::Serialize + de::DeserializeOwned,
{
    let state: &State = req.extensions().unwrap().get().unwrap();
    let header_str = req
        .headers()
        .unwrap()
        .get(http::header::AUTHORIZATION)
        .ok_or(TokenError::MissingHeader)?
        .to_str()
        .map_err(|err| AuthError::InvalidAuthorizationHeader(err.to_string()))?;

    let (schema, token) = header_str
        .split_once(' ')
        .ok_or_else(|| AuthError::InvalidAuthorizationHeader(String::from("invalid syntax")))?;

    if schema != "Bearer" {
        return Err(AuthError::InvalidAuthorizationHeader(schema.to_string()));
    }

    let token = Token::<P>::decode(get_key_fn(&state.config.secrets).as_bytes(), token)?;
    Ok(token)
}

#[async_trait]
impl axum::extract::FromRequest<Body> for RefreshToken {
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(
            from_request(req, |secrets| &secrets.refresh_key).await?,
        ))
    }
}

#[async_trait]
impl axum::extract::FromRequest<Body> for AccessToken
{
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(
            from_request(req, |secrets| &secrets.access_key).await?,
        ))
    }
}
