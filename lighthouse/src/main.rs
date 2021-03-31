use actix_web::{HttpServer, web, get, post, HttpResponse};
use uuid::Uuid;
use std::sync::Mutex;
use std::collections::HashMap;
use std::io;
use ws::WebsocketSession;

mod ws;
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

    Ok(HttpResponse::Ok().json(ExecuteResponse {
        status: ExecuteResponseStatus::Success,
        states: std::collections::HashMap::new(),
        error_code: None,
    }))
}


#[post("/execute")]
async fn execute(
    _request: web::Json<ExecuteRequest>,
) -> actix_web::Result<HttpResponse> {

    Ok(HttpResponse::Ok().json(ExecuteResponse {
        status: ExecuteResponseStatus::Success,
        states: std::collections::HashMap::new(),
        error_code: None,
    }))
}

#[get("/testing/{id}")]
async fn testing(
    app_state: web::Data<AppState>,
    path: web::Path<(String,)>
) -> actix_web::Result<HttpResponse> {
    log::info!("Received for testing");
    let (id,) = path.into_inner();
    let sessions = app_state.sessions.lock().unwrap();
    let session = sessions.get(&id);
    if session.is_none() {
        return Ok(HttpResponse::BadRequest().body(format!("Session not found with id: {}", id)));
    }

    let request = ExecuteRequest{
        params: HashMap::new(),
        command: "commandhere".to_string(),
    };
    let response = session.unwrap().send(request)
        .await
        .unwrap().0.await;

    println!("Execute response: {:?}", response);

    Ok(HttpResponse::Ok().body(format!("{:?}", response)))
}


pub struct AppState {
    sessions: Mutex<HashMap<String, actix::Addr<WebsocketSession>>>,
}


#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    log::info!("Starting houseflow-fulfillment");

    let app_state = web::Data::new(AppState {
        sessions: Mutex::new(HashMap::new())
    });

    HttpServer::new(move || {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(app_state.clone())
            .service(execute)
            .service(query)
            .service(testing)
            .service(ws::index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
    .unwrap();

    Ok(())
}

