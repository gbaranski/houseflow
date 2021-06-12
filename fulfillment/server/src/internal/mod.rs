use crate::ActixUser;
use actix_web::{get, HttpResponse, Responder};

#[get("/sync")]
pub async fn on_sync(user: ActixUser) -> impl Responder {
    let _user = user.inner;
    HttpResponse::Ok().body("syncing")
}
