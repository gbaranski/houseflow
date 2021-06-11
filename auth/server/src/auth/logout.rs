use crate::AppData;
use actix_web::{
    post,
    web::{Data, Json},
    HttpRequest,
};
use auth_types::LogoutResponseError;

use token::store::TokenStore;
use token::Token;

#[post("/logout")]
pub async fn logout(
    token_store: Data<dyn TokenStore>,
    app_data: Data<AppData>,
    req: HttpRequest,
) -> Result<Json<()>, LogoutResponseError> {
    let refresh_token = Token::from_request(&req)?;
    refresh_token.verify(&app_data.refresh_key, None)?;
    token_store.remove(refresh_token.id()).await.unwrap();
    Ok(Json(()))
}
