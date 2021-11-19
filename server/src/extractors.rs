use crate::State;
use async_trait::async_trait;
use axum::body::Body;
use houseflow_config::server::Secrets;
use houseflow_types::errors::AuthError;
use houseflow_types::errors::ServerError;
use houseflow_types::errors::TokenError;
use houseflow_types::token::AccessTokenPayload;
use houseflow_types::token::RefreshTokenPayload;
use houseflow_types::token::Token;
use houseflow_types::user;
use jsonwebtoken::TokenData;
use serde::de;
use serde::ser;

pub struct UserID(pub user::ID);

#[async_trait]
impl axum::extract::FromRequest<Body> for UserID {
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        let AccessToken(access_token) = AccessToken::from_request(req).await?;
        Ok(Self(access_token.claims.sub))
    }
}

pub struct RefreshToken(pub TokenData<RefreshTokenPayload>);
pub struct AccessToken(pub TokenData<AccessTokenPayload>);

async fn from_request<P>(
    req: &mut axum::extract::RequestParts<Body>,
    get_key_fn: impl FnOnce(&Secrets) -> &str,
) -> Result<TokenData<P>, AuthError>
where
    P: ser::Serialize + de::DeserializeOwned,
{
    let state: &State = req.extensions().unwrap().get().unwrap();
    let header_str = req
        .headers()
        .unwrap()
        .get(http::header::AUTHORIZATION)
        .ok_or(TokenError {
            description: "MissingHeader".to_string(),
        })?
        .to_str()
        .map_err(|err| AuthError::InvalidAuthorizationHeader(err.to_string()))?;

    let (schema, token) = header_str
        .split_once(' ')
        .ok_or_else(|| AuthError::InvalidAuthorizationHeader(String::from("invalid syntax")))?;

    if schema != "Bearer" {
        return Err(AuthError::InvalidAuthorizationHeader(schema.to_string()));
    }

    Ok(Token::<P>::decode(
        get_key_fn(&state.config.secrets).as_bytes(),
        token,
    )?)
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
impl axum::extract::FromRequest<Body> for AccessToken {
    type Rejection = ServerError;

    async fn from_request(
        req: &mut axum::extract::RequestParts<Body>,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self(
            from_request(req, |secrets| &secrets.access_key).await?,
        ))
    }
}
