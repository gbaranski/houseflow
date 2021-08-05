use crate::State;
use async_trait::async_trait;
use houseflow_config::server::Secrets;
use houseflow_types::{errors::{AuthError, ServerError, TokenError}, token::{AccessTokenPayload, RefreshTokenPayload, Token}};

use serde::{de, ser};

pub struct RefreshToken(pub Token<RefreshTokenPayload>);

impl std::ops::Deref for RefreshToken {
    type Target = Token<RefreshTokenPayload>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AccessToken(pub Token<AccessTokenPayload>);

impl std::ops::Deref for AccessToken {
    type Target = Token<AccessTokenPayload>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

async fn from_request<B, P>(
    req: &mut axum::extract::RequestParts<B>,
    get_key_fn: impl FnOnce(&Secrets) -> &str,
) -> Result<Token<P>, AuthError>
where
    B: http_body::Body + Send,
    B::Data: Send,
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
        return Err(AuthError::InvalidAuthorizationHeader(schema.to_string()).into());
    }

    let token = Token::<P>::decode(get_key_fn(&state.config.secrets).as_bytes(), token)?;
    Ok(token)
}

#[async_trait]
impl<B> axum::extract::FromRequest<B> for RefreshToken
where
    B: http_body::Body + Send,
    B::Data: Send,
{
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(from_request(req, |secrets| &secrets.refresh_key).await?))
    }
}

#[async_trait]
impl<B> axum::extract::FromRequest<B> for AccessToken
where
    B: http_body::Body + Send,
    B::Data: Send,
{
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(from_request(req, |secrets| &secrets.access_key).await?))
    }
}
