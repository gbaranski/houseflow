use houseflow_db::Database;
use actix_web::{web, post, App, HttpResponse, HttpRequest, HttpServer, Responder};
use crate::error::Error;

mod error;
mod intent {
    mod request;
    mod sync;
    mod execute;
    mod query;

    pub(crate) use request::Request;
    pub(crate) use execute::ExecutePayload;
    pub(crate) use query::QueryPayload;
}


/// This struct represents shared state across routes
#[derive(Clone)]
struct AppState {
    db: Database,
}



#[post("/webhook")]
async fn webhook(_req: HttpRequest, intent_request: web::Json<intent::Request>, _data: web::Data<AppState>) -> impl Responder {
    log::info!("Request ID: {}", intent_request.request_id);
    HttpResponse::InternalServerError()
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


