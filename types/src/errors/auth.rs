use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum Error {
    /// `Authorization` header has invalid syntax
    #[error("invalid authorization header: {0}")]
    InvalidAuthorizationHeader(String),
    /// Client sent invalid token
    #[error("invalid token: {0}")]
    InvalidToken(#[from] super::token::Error),
    /// Client sent invalid verification code
    #[error("invalid verification code: {0}")]
    InvalidVerificationCode(String),
    /// When password hashes doesn't match with the one from database
    #[error("invalid password")]
    InvalidPassword,
    /// When client tries to authenticate, but user with given credentails have not been found
    #[error("user not found")]
    UserNotFound,
    /// When hub tries to authenticate, but hub with given credentails has not been found
    #[error("device not found")]
    HubNotFound,
    /// Occurs when user tries to register, but user with given credentials already exists
    #[error("user already exists")]
    UserAlreadyExists,
    /// Refresh token is blacklisted
    #[error("refresh token is blacklisted")]
    RefreshTokenBlacklisted,
    /// User does not have permission to a device
    #[error("user does not have permission to a specified device")]
    NoDevicePermission,
    /// Invalid Google JWT
    #[error("invalid Google JWT: {0}")]
    InvalidGoogleJwt(String),
    /// The CSRF token cookie was missing, or didn't match the token in the request.
    #[error("Missing or invalid CSRF token")]
    InvalidCsrfToken,
    /// User does not have permission to a structure
    #[error("user does not have permission to a specified structure")]
    NoStructurePermission,
}
