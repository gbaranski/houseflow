use actix_web::{
    post,
    web::{self, Json},
};
pub use config::Config;
use connect::on_websocket;
use db::Database;
use lighthouse_types::{DeviceError, ExecuteRequest, ExecuteResponse, ExecuteResponseBody};
use session::Session;
use std::collections::HashMap;
use tokio::sync::Mutex;
use types::DeviceID;
use lighthouse_proto::execute_response;

mod aliases;
pub mod config;
mod connect;
mod session;

#[post("/execute")]
async fn on_execute(
    request: Json<ExecuteRequest>,
    app_state: web::Data<AppState>,
) -> Result<Json<ExecuteResponse>, DeviceError> {
    let sessions = app_state.sessions.lock().await;
    let session = sessions.get(&request.device_id).ok_or(DeviceError::NotConnected)?;
    let response_frame = session.send(aliases::ActorExecuteFrame::from(request.frame.clone())).await.unwrap()?;
    let response_frame = execute_response::Frame::from(response_frame);
    let response = ExecuteResponse::Ok(ExecuteResponseBody {
        frame: response_frame,
    });

    log::debug!("Response: {:?}", response);
    Ok(Json(response))
}

#[derive(Default)]
pub struct AppState {
    sessions: Mutex<HashMap<DeviceID, actix::Addr<Session>>>,
}

pub fn configure(
    cfg: &mut web::ServiceConfig,
    app_state: web::Data<AppState>,
    database: web::Data<dyn Database>,
) {
    cfg.app_data(app_state)
        .app_data(database)
        .service(on_websocket)
        .service(
            web::scope("/")
                .guard(actix_web::guard::Host("127.0.0.1"))
                .service(on_execute),
        );
}
