use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use auth_types::{LogoutResponse, LogoutResponseBody, LogoutResponseError};
use config::server::Secrets;
use token::store::TokenStore;
use token::Token;

#[post("/logout")]
pub async fn on_logout(
    token_store: Data<dyn TokenStore>,
    secrets: Data<Secrets>,
    req: HttpRequest,
) -> Result<Json<LogoutResponse>, LogoutResponseError> {
    let refresh_token = Token::from_request(&req)?;
    refresh_token.verify(&secrets.refresh_key, None)?;
    let removed = token_store.remove(refresh_token.id()).await.unwrap();
    let response = LogoutResponseBody {
        token_removed: removed,
    };
    Ok(Json(LogoutResponse::Ok(response)))
}
