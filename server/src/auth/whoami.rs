use crate::{extractors::AccessToken, Error, State};
use axum::{extract, response};
use houseflow_types::auth::whoami::Response;
use tracing::Level;

#[tracing::instrument(name = "Whoami", skip(state, access_token))]
pub async fn handle(
    extract::Extension(state): extract::Extension<State>,
    AccessToken(access_token): AccessToken,
) -> Result<response::Json<Response>, Error> {
    let user = state
        .database
        .get_user(&access_token.sub)?
        .ok_or(Error::UserNotFound)?;

    tracing::event!(Level::INFO, user_id = %user.id);

    Ok(response::Json(Response {
        username: user.username,
        email: user.email,
    }))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::test_utils::*;
//     use actix_web::{http, test};
//     use chrono::{Duration, Utc};
//     use houseflow_types::{token::AccessTokenPayload, User};
//
//     use rand::random;
//
//     #[actix_rt::test]
//     async fn valid() {
//         let state = get_state();
//         let user = User {
//             id: random(),
//             username: String::from("John Smith"),
//             email: String::from("john_smith@example.com"),
//             password_hash: PASSWORD_HASH.into(),
//         };
//
//         let access_token = AccessToken::new(
//             state.config.secrets.access_key.as_bytes(),
//             AccessTokenPayload {
//                 sub: user.id.clone(),
//                 exp: Utc::now() + Duration::seconds(5),
//             },
//         );
//
//         state.database.add_user(&user).unwrap();
//
//         let request = test::TestRequest::default()
//             .append_header((
//                 http::header::AUTHORIZATION,
//                 format!("Bearer {}", access_token.to_string()),
//             ))
//             .to_http_request();
//         let response = on_whoami(state.config, state.database.clone(), request)
//             .await
//             .unwrap()
//             .into_inner();
//
//         assert_eq!(user.email, response.email);
//         assert_eq!(user.username, response.username);
//     }
//
//     #[actix_rt::test]
//     async fn missing_token() {
//         let state = get_state();
//         let request = test::TestRequest::default().to_http_request();
//         let response = on_whoami(state.config, state.database, request)
//             .await
//             .unwrap_err();
//         assert!(matches!(response, ResponseError::TokenError(_)));
//     }
//
//     #[actix_rt::test]
//     async fn invalid_token_signature() {
//         let state = get_state();
//         let user = get_user();
//         let access_token = AccessToken::new(
//             &Vec::from("other key"),
//             AccessTokenPayload {
//                 sub: user.id.clone(),
//                 exp: Utc::now() + Duration::seconds(5),
//             },
//         );
//
//         state.database.add_user(&user).unwrap();
//         let request = test::TestRequest::default()
//             .append_header((
//                 http::header::AUTHORIZATION,
//                 format!("Bearer {}", access_token.to_string()),
//             ))
//             .to_http_request();
//         let response = on_whoami(state.config, state.database, request)
//             .await
//             .unwrap_err();
//
//         assert!(matches!(response, ResponseError::TokenError(_)));
//     }
// }
