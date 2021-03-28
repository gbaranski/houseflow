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

    pub use intent::{Request, RequestPayload, Response, ResponsePayload};
    pub use error::IntentError;
}


/// This struct represents shared state across routes
#[derive(Clone)]
pub struct AppState {
    db: Database,
    mc: memcache::Client,
}

impl AppState {
    pub fn lighthouse(&self) -> LighthouseAPI {
        LighthouseAPI::new(&self.mc, &self.db)
    }
}


#[post("/webhook")]
async fn webhook<'a>(
    req: HttpRequest,
    intent_request: web::Json<intent::Request>,
    app_state: web::Data<AppState>
) -> Result<web::HttpResponse, Error> {
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

    let user = app_state.db.get_user_by_id(access_token.payload.audience)
        .await?
        .ok_or(AuthError::UserNotFound)?;


    // Thats fixed because Google has weird API
    let request_input = &intent_request.inputs[0];

    let response_payload = match &request_input.payload {
        intent::RequestPayload::Sync()     => intent::sync   ::handle(&app_state, &user, ()).await,
        intent::RequestPayload::Execute(p) => intent::execute::handle(&app_state, &user, p).await,
    };

    Ok(HttpResponse::Ok().json(intent::Response {
        request_id: intent_request.request_id.clone(),
        payload: response_payload,
    }))
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


