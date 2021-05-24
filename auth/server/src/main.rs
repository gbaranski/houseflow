use actix_web::{post, web, App, HttpServer};
use houseflow_auth_types::{
    AccessTokenRequestError, AccessTokenRequestErrorKind, AccessTokenResponseBody, GrantType,
    TokenType,
};
use houseflow_token::{
    ExpirationDate, Payload as TokenPayload, Signature as TokenSignature, Token, TokenID,
};
use houseflow_types::{UserAgent, UserID};
use token_store::TokenStore;

mod token_store;

#[post("/token")]
async fn token(// req: web::Query<AccessTokenRequestBody>,
    // state: web::Data<AppState>,
) -> Result<web::Json<AccessTokenResponseBody>, AccessTokenRequestError> {
    use std::convert::TryFrom;
    use std::time::{Duration, SystemTime};
    let expires_in = Duration::from_secs(3600);
    let payload = TokenPayload {
        id: TokenID::try_from("1b83055496544bc4873b40054529417f").unwrap(),
        user_agent: UserAgent::GoogleSmartHome,
        user_id: UserID::try_from("476f5fbe25824291a5a87d8097071321").unwrap(),
        expires_at: ExpirationDate::from(
            SystemTime::now().checked_add(expires_in.clone()).unwrap(),
        ),
    };
    let signature = payload.sign(b"some-key");
    let token = Token::new(payload, signature);
    Ok(web::Json(AccessTokenResponseBody {
        access_token: token,
        token_type: TokenType::Bearer,
        // TODO: Fix serializing ExpiresIn
        expires_in: Some(expires_in),
    }))
    // Err(AccessTokenRequestError {
    //     error: AccessTokenRequestErrorKind::InvalidClient,
    //     error_description: Some("test".into()),
    // })
}

#[derive(Clone)]
pub struct AppState {
    token_store: TokenStore,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const IP_ADDR: &str = "127.0.0.1:8080";
    env_logger::init();
    log::info!("Starting `Auth` service");

    let server = HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(token)
    })
    .bind(IP_ADDR)?;
    log::info!("Starting HTTP Server at `{}`", IP_ADDR);
    server.run().await?;
    Ok(())
}
