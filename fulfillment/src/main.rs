use houseflow_db::Database;
use houseflow_lighthouse::LighthouseAPI;
use houseflow_token::Token;
use actix_web::{web, post, App, HttpResponse, HttpRequest, HttpServer};
use error::{Error, AuthError};

mod error;
mod intent {
    mod intent;
    mod error;
    pub mod sync;
    pub mod execute;
    pub mod query;

    pub use intent::Request;
    pub use error::IntentError;
}


/// This struct represents shared state across routes
#[derive(Clone)]
pub struct AppState<'a> {
    db: Database,
    memcache: memcache::Client,
    lighthouse: LighthouseAPI<'a>,
}


#[post("/webhook")]
async fn webhook<'a>(
    req: HttpRequest,
    intent_request: web::Json<intent::Request>,
    state: web::Data<AppState<'a>>
) -> Result<&'static str, Error> {
    let access_token_base64 = match req.headers().get("Authorization") {
        Some(value) => {
            // Bearer XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
            // 01234567...
            // Token starts at byte 7
            Ok(&value.as_bytes()[7..])
        },
        None => Err(AuthError::MissingToken),
    }?;

    let access_token = Token::from_base64(access_token_base64)?;
    access_token.verify(std::env::var("ACCESS_KEY").unwrap().as_bytes())?;

    let user = state.db.get_user_by_id(access_token.payload.audience)
        .await?
        .ok_or(AuthError::UserNotFound)?;


    // intent_request.inputs
    //     .iter()
    //     .map(|input| match input.intent {
    //         // "action.devices.SYNC" => intent::sync::handle()
    //     });
    log::info!("Request ID: {}", intent_request.request_id);
    HttpResponse::InternalServerError();
    Ok("sadhahs")
}



#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    log::info!("Starting houseflow-fulfillment");

    let db = Database::connect()?;
    db.init().await?;

    let memcache = memcache::connect("memcache://memcache:11211?timeout=10&tcp_nodelay=true")?;
    
    let app_state = AppState {
        db,
        memcache,
        lighthouse: LighthouseAPI {
            memcache: &memcache,
            db: &db,
        },
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


