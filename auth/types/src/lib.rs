use std::time::Duration;
use houseflow_token::Token;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename = "snake_case"))]
pub enum GrantType {
    RefreshToken,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AccessTokenRequest {
    /// The grant_type parameter must be set to `GrantType::RefreshToken`.
    grant_type: GrantType,

    /// The refresh token previously issued to the client.
    refresh_token: Token,
}


#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum TokenType {
    Bearer,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AccessTokenResponse {
    /// The access token string as issued by the authorization server.
    access_token: Token,

    /// The type of token this is, typically just the string “Bearer”.
    token_type: TokenType,
    
    /// If the access token expires, the server should reply with the duration of time the access token is granted for.
    expires_in: Duration,
}
