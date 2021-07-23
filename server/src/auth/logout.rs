use crate::TokenStore;
use actix_web::{
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_types::{
    auth::logout::{ResponseBody, ResponseError},
    token::RefreshToken,
};
use tracing::Level;

#[tracing::instrument(name = "Logout", skip(token_store, config, http_request))]
pub async fn on_logout(
    token_store: Data<dyn TokenStore>,
    config: Data<Config>,
    http_request: HttpRequest,
) -> Result<Json<ResponseBody>, ResponseError> {
    let refresh_token =
        RefreshToken::from_request(config.secrets.refresh_key.as_bytes(), &http_request)?;
    token_store.remove(&refresh_token.tid).await.unwrap();
    tracing::event!(Level::INFO, user_id = %refresh_token.sub);
    Ok(Json(ResponseBody {}))
}
