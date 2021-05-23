use thiserror::Error;

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

    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let headers = req.headers();
        let agent_state = req.app_data::<AgentState>().unwrap();
        let token_key = agent_state.token_key.clone();
        let expected_user_agent = agent_state.users_agent.clone();
        let database = agent_state.database;
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
            let user_id = Token::from_base64(token)
                .map_err(AuthorizationError::InvalidToken)?
                .verify(&token_key, &expected_user_agent)
                .map_err(AuthorizationError::InvalidToken)?
                .payload
                .user_id;

            let user = database
                .get_user(&user_id)
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
async fn main() -> Result<(), Error> {
    env_logger::init();
    log::info!("Starting houseflow-fulfillment");

    let db = Database::connect()?;
    db.init().await?;

    let mc = memcache::connect("memcache://memcache:11211?timeout=10&tcp_nodelay=true")?;
    
    let app_state = AppState {
        db,
        mc,
    };

    log::info!("Starting HttpServer");
    HttpServer::new(move || {
        App::new()
            .data(app_state.to_owned())
            .wrap(actix_web::middleware::Logger::default())
            .service(webhook)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
    .unwrap();

    Ok(())
}
