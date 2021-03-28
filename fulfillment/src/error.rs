use serde::Serialize;
use memcache::MemcacheError;
use actix_web::{dev::HttpResponseBuilder, HttpResponse, http::StatusCode};


#[derive(Debug)]
pub enum AuthError {
    UserNotFound,
    MissingToken,
    InvalidToken(houseflow_token::Error),
}


impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            AuthError::UserNotFound => "user not found".to_string(),
            AuthError::MissingToken => "token is missing".to_string(),
            AuthError::InvalidToken(err) => err.to_string(),

        };

        write!(f, "{}", msg)
    }
}

impl From<houseflow_token::Error> for AuthError {
    fn from(err: houseflow_token::Error) -> Self {
        Self::InvalidToken(err)
    }
}


#[derive(Debug)]
pub enum Error {
    AuthError(AuthError),

    DatabaseError(houseflow_db::Error),
    MemcacheError(MemcacheError),
    IOError(std::io::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    pub error: String,
}

impl From<houseflow_db::Error> for Error {
    fn from(err: houseflow_db::Error) -> Error {
        Error::DatabaseError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<MemcacheError> for Error {
    fn from(err: MemcacheError) -> Error {
        Error::MemcacheError(err)
    }
}


impl From<AuthError> for Error {
    fn from(err: AuthError) -> Self {
        Self::AuthError(err)
    }
}

impl From<houseflow_token::Error> for Error {
    fn from(err: houseflow_token::Error) -> Self {
        Self::AuthError(AuthError::InvalidToken(err))
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::AuthError(err) => err.to_string(),

            Error::DatabaseError(err) => err.to_string(),
            Error::IOError(err) => err.to_string(),
            Error::MemcacheError(err) => err.to_string(),
        };

        write!(f, "{}", msg)
    }
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use Error::{DatabaseError, IOError, MemcacheError};

        match self {
            Error::AuthError(auth_err) => match auth_err {
                AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
                AuthError::MissingToken => StatusCode::BAD_REQUEST,
                AuthError::UserNotFound => StatusCode::BAD_REQUEST,
            },
            MemcacheError(_err) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseError(_err) => StatusCode::INTERNAL_SERVER_ERROR,
            IOError(_err) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .json(ErrorResponse{
                error: self.to_string(),
            })
    }

}
