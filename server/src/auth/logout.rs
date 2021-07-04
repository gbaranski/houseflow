use crate::TokenStore;
use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_types::{
    auth::logout::{ResponseBody, ResponseError},
    token::RefreshToken,
};

#[post("/logout")]
pub async fn on_logout(
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
    http_request: HttpRequest,
) -> Result<Json<ResponseBody>, ResponseError> {
    let refresh_token = RefreshToken::from_request(&config.secrets.refresh_key, &http_request)?;
    token_store.remove(&refresh_token.tid).await.unwrap();
    Ok(Json(ResponseBody {}))
}
