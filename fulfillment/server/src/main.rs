use actix_web::{
    http,
    web::{self, Data},
    App, HttpRequest, HttpServer,
};
use db::{Database, Error as DatabaseError, MemoryDatabase};
use token::Token;
use types::{User, UserAgent};
use thiserror::Error;

mod gactions;
mod internal;

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("missing authorization header in request")]
    MissingHeader,

    #[error("authorization header has invalid ASCII Characters")]
    InvalidHeaderEncoding(#[from] http::header::ToStrError),

    #[error("authorization header has invalid syntax")]
    InvalidHeaderSyntax,

    #[error("authorization header does not have or have invalid schema")]
    InvalidHeaderSchema,

    #[error("token is invalid: `{0}`")]
    InvalidToken(#[from] token::Error),

    #[error("user has not been found in database")]
    UserNotFound,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("authorization error: `{0}`")]
    Authorization(#[from] AuthorizationError),

    #[error("database error: `{0}`")]
    DatabaseError(#[from] DatabaseError),
}

impl actix_web::ResponseError for Error {}

pub struct ActixUser {
    inner: User,
}

impl From<User> for ActixUser {
    fn from(user: User) -> Self {
        Self { inner: user }
    }
}

impl Into<User> for ActixUser {
    fn into(self) -> User {
        self.inner
    }
}

use std::{future::Future, pin::Pin};

impl actix_web::FromRequest for ActixUser {
    type Config = ();

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let headers = req.headers();
        let access_key = req.app_data::<AppData>().unwrap().access_key.clone();
        let database = req.app_data::<Data<dyn Database>>().unwrap().clone();
        let expected_user_agent = req.app_data::<AgentData>().unwrap().user_agent;
        let authorization_header = match headers
            .get(http::header::AUTHORIZATION)
            .ok_or(AuthorizationError::MissingHeader)
        {
            Ok(header) => header,
            Err(err) => return Box::pin(async { Err(err.into()) }),
        }
        .clone();

        let fut = async move {
            let (schema, token) = authorization_header
                .to_str()
                .map_err(AuthorizationError::InvalidHeaderEncoding)
                .unwrap()
                .split_once(' ')
                .ok_or(AuthorizationError::InvalidHeaderSyntax)
                .unwrap();
            if schema != "Bearer" {
                return Err(AuthorizationError::InvalidHeaderSchema.into());
            }
            let token = Token::from_str(token)
                .map_err(|err| AuthorizationError::InvalidToken(err.into()))?;

            token
                .verify(&access_key, Some(&expected_user_agent))
                .map_err(|err| AuthorizationError::InvalidToken(err.into()))?;

            let user = database
                .get_user(&token.user_id())
                .await?
                .ok_or(AuthorizationError::UserNotFound)?;

            Ok(user.into())
        };

        Box::pin(fut)
    }
}

#[derive(Clone)]
pub struct AppData {
    access_key: Vec<u8>,
    refresh_key: Vec<u8>,
}

#[derive(Clone)]
pub struct AgentData {
    user_agent: UserAgent,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting fulfillment service");

    let database = MemoryDatabase::new();
    log::info!("Database initialized");

    let app_data = AppData {
        refresh_key: Vec::from("refresh-key"),
        access_key: Vec::from("access-key"),
    };

    let internal_agent_data = AgentData {
        user_agent: UserAgent::Internal,
    };

    // let google_actions_agent_data = AgentData {
    //     user_agent: UserAgent::GoogleSmartHome,
    // };

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .data(app_data.clone())
            .app_data(database.clone())
            .service(
                web::scope("/internal")
                    .app_data(internal_agent_data.clone())
                    .service(internal::on_sync),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
    .unwrap();

    Ok(())
}
