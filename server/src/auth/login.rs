use crate::State;
use axum::{extract, response};
use chrono::{Duration, Utc};
use houseflow_types::{
    auth::login::{Request, Response},
    errors::{AuthError, ServerError},
    token::{AccessToken, AccessTokenPayload, RefreshToken, RefreshTokenPayload},
};
use tracing::Level;

#[tracing::instrument(
    name = "Login",
    skip(state, request),
    fields(
        email = %request.email,
    ),
    err,
)]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    extract::Json(request): extract::Json<Request>,
) -> Result<response::Json<Response>, ServerError> {
    validator::Validate::validate(&request)?;
    let user = state
        .database
        .get_user_by_email(&request.email)?
        .ok_or(AuthError::UserNotFound)?;

    crate::verify_password(&user.password_hash, &request.password)?;
    let refresh_token = RefreshToken::new(
        state.config.secrets.refresh_key.as_bytes(),
        RefreshTokenPayload {
            sub: user.id.clone(),
            exp: Some(Utc::now() + Duration::days(7)),
            tid: rand::random(),
        },
    );
    let access_token = AccessToken::new(
        state.config.secrets.access_key.as_bytes(),
        AccessTokenPayload {
            sub: user.id.clone(),
            exp: Utc::now() + Duration::minutes(10),
        },
    );
    state
        .token_store
        .add(&refresh_token.tid, refresh_token.exp.as_ref())
        .await?;

    tracing::event!(Level::INFO, user_id = %user.id, "Logged in");

    Ok(response::Json(Response {
        access_token: access_token.encode(),
        refresh_token: refresh_token.encode(),
    }))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_utils::*;
//
//     #[actix_rt::test]
//     async fn valid() {
//         let state = get_state();
//         let user = get_user();
//         state.database.add_user(&user).unwrap();
//         let response = on_login(
//             Json(Request {
//                 email: user.email,
//                 password: PASSWORD.into(),
//             }),
//             state.token_store.clone(),
//             state.config.clone(),
//             state.database,
//         )
//         .await
//         .unwrap()
//         .into_inner();
//
//         let (at, rt) = (response.access_token, response.refresh_token);
//         let (at, rt) = (
//             AccessToken::decode(state.config.secrets.access_key.as_bytes(), &at).unwrap(),
//             RefreshToken::decode(state.config.secrets.refresh_key.as_bytes(), &rt).unwrap(),
//         );
//         assert_eq!(at.sub, rt.sub);
//         assert!(
//             state.token_store.exists(&rt.tid).await.unwrap(),
//             "refresh token not found in token store"
//         );
//     }
//
//     #[actix_rt::test]
//     async fn invalid_password() {
//         let state = get_state();
//         let user = get_user();
//         state.database.add_user(&user).unwrap();
//         let response = on_login(
//             Json(Request {
//                 email: user.email,
//                 password: PASSWORD_INVALID.into(),
//             }),
//             state.token_store.clone(),
//             state.config.clone(),
//             state.database,
//         )
//         .await
//         .unwrap_err();
//
//         assert_eq!(response, ResponseError::InvalidPassword);
//     }
//
//     #[actix_rt::test]
//     async fn not_existing_user() {
//         let state = get_state();
//         let user = get_user();
//         let response = on_login(
//             Json(Request {
//                 email: user.email,
//                 password: PASSWORD.into(),
//             }),
//             state.token_store.clone(),
//             state.config.clone(),
//             state.database,
//         )
//         .await
//         .unwrap_err();
//
//         assert_eq!(response, ResponseError::UserNotFound);
//     }
// }
