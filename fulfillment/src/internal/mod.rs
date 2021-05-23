use crate::{AgentState, ActixUser};
use actix_web::{get, web, HttpResponse, Responder};


#[get("/sync")]
pub async fn on_sync(_app_state: web::Data<AgentState>, user: ActixUser) -> impl Responder {
    let _user = user.inner;
    HttpResponse::Ok().body("syncing")
}
