use crate::{token_store::Error as TokenStoreError, TokenStore};
use actix_web::web::{Data, Json};
use chrono::{Duration, Utc};
use houseflow_config::server::Config;
use houseflow_types::{
    auth::token::{Request, ResponseBody, ResponseError},
    token::{AccessToken, AccessTokenPayload, RefreshToken},
};

pub async fn on_refresh_token(
    config: Data<Config>,
    token_store: Data<dyn TokenStore>,
    Json(request): Json<Request>,
) -> Result<Json<ResponseBody>, ResponseError> {
    let refresh_token = RefreshToken::decode(
        config.secrets.refresh_key.as_bytes(),
        &request.refresh_token,
    )?;
    if !token_store
        .exists(&refresh_token.tid)
        .await
        .map_err(TokenStoreError::into_internal_server_error)?
    {
        return Err(ResponseError::TokenNotInStore);
    }

    let access_token_payload = AccessTokenPayload {
        sub: refresh_token.sub.clone(),
        exp: Utc::now() + Duration::minutes(10),
    };
    let access_token = AccessToken::new(config.secrets.access_key.as_bytes(), access_token_payload);
    Ok(Json(ResponseBody {
        refresh_token: None,
        access_token: access_token.to_string(),
    }))
}
