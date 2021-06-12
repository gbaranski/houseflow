use actix_web::{get, HttpResponse, Responder};
use token::Token;

#[get("/sync")]
pub async fn on_sync(request: actix_web::HttpRequest) -> impl Responder {
    let refresh_token = Token::from_request(&request);
    HttpResponse::Ok().body("syncing")
}
