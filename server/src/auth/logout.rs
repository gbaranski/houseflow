use crate::TokenStore;
use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_types::{
    auth::{LogoutResponse, LogoutResponseBody, LogoutResponseError},
    token::Token,
};

#[post("/logout")]
pub async fn on_logout(
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
    req: HttpRequest,
) -> Result<Json<LogoutResponse>, LogoutResponseError> {
    let refresh_token = Token::from_request(&req)?;
    refresh_token.verify(&config.secrets.refresh_key, None)?;
    let removed = token_store.remove(refresh_token.id()).await.unwrap();
    let response = LogoutResponseBody {
        token_removed: removed,
    };
    Ok(Json(LogoutResponse::Ok(response)))
}
