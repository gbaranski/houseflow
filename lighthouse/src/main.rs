use actix_web::{HttpServer, web, get, post, HttpResponse};
use uuid::Uuid;
use std::io;

mod types;
use types::*;

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}


#[get("/query/{device_id}")]
async fn query(
    path: web::Path<(Uuid,)>
) -> actix_web::Result<HttpResponse> {
    let (device_id,) = path.into_inner();
    log::info!("Querying device ID: {}", device_id);

    Ok(HttpResponse::Ok().json(Response {
        status: ResponseStatus::Success,
        states: std::collections::HashMap::new(),
        error_code: None,
    }))
}


#[post("/execute")]
async fn execute(
    request: web::Json<ExecuteRequest>,
) -> actix_web::Result<HttpResponse> {
    log::info!("Execute for device ID: {}", request.device_id);

    Ok(HttpResponse::Ok().json(Response {
        status: ResponseStatus::Success,
        states: std::collections::HashMap::new(),
        error_code: None,
    }))
}



#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    log::info!("Starting houseflow-fulfillment");


    HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(execute)
            .service(query)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
    .unwrap();

    Ok(())
}
