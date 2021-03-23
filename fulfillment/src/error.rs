use serde::Serialize;
use actix_web::{dev::HttpResponseBuilder, HttpResponse, http::StatusCode};

#[derive(Debug)]
pub enum Error {
    MissingToken,
    InvalidToken(String),
    UserNotFound,
    DatabaseError(houseflow_db::Error),
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

impl From<houseflow_token::Error> for Error {
    fn from(err: houseflow_db::Error) -> Error {
        Error::DatabaseError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::MissingToken => "missing_token".to_string(),
            Error::InvalidToken(err) => err.clone(),

            Error::UserNotFound => "user not found".to_string(),

            Error::DatabaseError(err) => err.to_string(),
            Error::IOError(err) => err.to_string(),
        };

        write!(f, "{}", msg)
    }
}
impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        use Error::*;
        match self {
            MissingToken => StatusCode::UNAUTHORIZED,
            InvalidToken(_err) => StatusCode::UNAUTHORIZED,

            UserNotFound => StatusCode::UNAUTHORIZED,

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
