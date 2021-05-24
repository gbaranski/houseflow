use actix_web::{http, web, App, HttpRequest, HttpServer};
use houseflow_db::{Database, Error as DatabaseError, Options as DatabaseOptions};
use houseflow_token::Token;
use houseflow_types::{User, UserAgent};
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
    InvalidToken(#[from] houseflow_token::Error),

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
        let agent_state = req.app_data::<AgentState>().unwrap();
        let token_key = agent_state.token_key.clone();
        let expected_user_agent = agent_state.users_agent.clone();
        let database = agent_state.database.clone();
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
            let token = Token::from_base64(token).map_err(AuthorizationError::InvalidToken)?;
            token
                .verify(&token_key, &expected_user_agent)
                .map_err(AuthorizationError::InvalidToken)?;

            let user = database
                .get_user(&token.payload.user_id)
                .await?
                .ok_or(AuthorizationError::UserNotFound)?;

            Ok(user.into())
        };

        Box::pin(fut)
    }
}

#[derive(Clone)]
pub struct AgentState {
    database: Database,
    users_agent: UserAgent,
    token_key: Vec<u8>,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    log::info!("Starting fulfillment service");

    // TODO: Replace those fixed values
    let database_options = DatabaseOptions {
        user: "postgres",
        password: "haslo123",
        host: "localhost",
        port: 5432,
        database_name: "houseflow",
    };
    let token_key = b"4a92c480aa4147ed-a3c36e5e667d8fbd";
    let database = Database::new(&database_options).await?;
    log::info!("Database initialized");

    let common_agent_state = AgentState {
        database,
        token_key: token_key.to_vec(),
        users_agent: UserAgent::default(),
    };

    let internal_agent_state = AgentState {
        users_agent: UserAgent::Internal,
        ..common_agent_state.clone()
    };

    let _google_actions_agent_state = AgentState {
        users_agent: UserAgent::GoogleSmartHome,
        ..internal_agent_state.clone()
    };

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // .wrap(auth)
            .service(
                web::scope("/internal")
                    .app_data(internal_agent_state.clone())
                    .service(internal::on_sync),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
    .unwrap();

    Ok(())
}
