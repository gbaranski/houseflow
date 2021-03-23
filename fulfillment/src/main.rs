use houseflow_db::{Database, models::User};
use actix_web::{web, post, App, HttpResponse, HttpRequest, HttpServer, Responder};
use std::convert::TryInto;
use houseflow_token::{Token, SizedFrame};
use crate::error::Error;

mod error;
mod intent {
    mod intent;
    pub mod sync;
    pub mod execute;
    pub mod query;

    pub use intent::Request;
}


/// This struct represents shared state across routes
#[derive(Clone)]
struct AppState {
    db: Database,
}


#[post("/webhook")]
async fn webhook(
    req: HttpRequest,
    intent_request: web::Json<intent::Request>,
    state: web::Data<AppState>
) -> Result<&'static str, Error> {
    let access_token_base64 = match req.headers().get("Authorization") {
        Some(value) => {
            // Bearer XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
            // 01234567...
            // Token starts at byte 7
            Ok(&value.as_bytes()[7..])
        },
        None => Err(Error::MissingToken),
    }?;
    let access_token_bytes: [u8; Token::SIZE] = base64::decode(access_token_base64)
        .map_err(|err| Error::InvalidToken(format!("fail decode base64: `{}`", err)))?
        .try_into()
        .map_err(|_err| Error::InvalidToken("failed conversion into fixed size array".to_string()))?;

    let access_token = Token::from_bytes(access_token_bytes);
    access_token.verify(std::env::var("ACCESS_KEY").unwrap().as_bytes())?;

    let user = state.db.user_by_id(access_token.payload.audience)
        .await?
        .ok_or(Error::UserNotFound)?;


    intent_request.inputs
        .iter()
        .map(|input| match input.intent {
            // "action.devices.SYNC" => intent::sync::handle()
        });
    log::info!("Request ID: {}", intent_request.request_id);
    HttpResponse::InternalServerError();
    Ok("sadhahs")
}



#[actix_web::main]
async fn main() -> Result<(), Error> {
    log::info!("Starting houseflow-fulfillment");
    env_logger::init();

    let db = Database::connect()?;
    db.init().await?;
    
    let app_state = AppState {
        db,
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
    .await?;

    Ok(())
}


