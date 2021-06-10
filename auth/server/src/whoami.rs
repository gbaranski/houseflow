use crate::AppData;
use actix_web::{
    get, http,
    web::{Data, Json},
    HttpRequest,
};
use auth_types::{WhoamiError, WhoamiResponse, WhoamiResponseBody};
use db::Database;
use token::Token;
use types::UserAgent;

#[get("/whoami")]
pub async fn whoami(
    req: HttpRequest,
    app_data: Data<AppData>,
    db: Data<dyn Database>,
) -> Result<Json<WhoamiResponse>, WhoamiError> {
    let authorization_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or(WhoamiError::MissingAuthorizationHeader)?;

    let (schema, token) = authorization_header
        .to_str()
        .map_err(|err| WhoamiError::InvalidHeaderEncoding(err.to_string()))?
        .split_once(' ')
        .ok_or(WhoamiError::InvalidHeaderSyntax)?;

    if schema != "Bearer" {
        return Err(WhoamiError::InvalidHeaderSchema(schema.to_string()));
    }
    let token = Token::from_str(token).map_err(|err| WhoamiError::InvalidToken(err.into()))?;
    token
        .verify(&app_data.access_key, Some(&UserAgent::Internal))
        .map_err(|err| WhoamiError::InvalidToken(err.into()))?;

    let user = db
        .get_user(token.user_id())
        .await
        .map_err(|err| WhoamiError::InternalError(err.to_string()))?
        .ok_or(WhoamiError::UserNotFound)?;

    let response = WhoamiResponseBody {
        username: user.username,
        email: user.email,
    };

    Ok(Json(WhoamiResponse::Ok(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;
    use actix_web::{test, App, ResponseError};
    use types::User;

    use rand::random;

    #[actix_rt::test]
    async fn valid() {
        let database = get_database();
        let app_data = get_app_data();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        let token = Token::new_access_token(&app_data.access_key, &user.id, &UserAgent::Internal);
        database.add_user(&user).await.unwrap();
        let mut app =
            test::init_service(App::new().configure(|cfg| {
                crate::config(cfg, get_token_store(), database, app_data.clone())
            }))
            .await;

        let request = test::TestRequest::get()
            .uri("/whoami")
            .append_header((
                http::header::AUTHORIZATION,
                format!("Bearer {}", token.to_string()),
            ))
            .to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(
            response.status(),
            200,
            "status is not succesfull, body: {:?}",
            test::read_body(response).await
        );
        let response: WhoamiResponse = test::read_body_json(response).await;
        let response = match response {
            Ok(response) => response,
            Err(err) => panic!("unexpected error: {}", err),
        };
        assert_eq!(user.email, response.email);
        assert_eq!(user.username, response.username);
    }


    #[actix_rt::test]
    async fn missing_header() {
        let database = get_database();
        let app_data = get_app_data();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        database.add_user(&user).await.unwrap();
        let mut app =
            test::init_service(App::new().configure(|cfg| {
                crate::config(cfg, get_token_store(), database, app_data.clone())
            }))
            .await;

        let request = test::TestRequest::get().uri("/whoami").to_request();
        let response = test::call_service(&mut app, request).await;

        assert_eq!(
            response.status(),
            WhoamiError::MissingAuthorizationHeader.status_code(),
            "unexpected status: {}, body: {:?}",
            response.status(),
            test::read_body(response).await
        );
        let response: WhoamiResponse = test::read_body_json(response).await;
        match response {
            Err(WhoamiError::MissingAuthorizationHeader) => (),
            _ => panic!("unexpected response: {:?}", response),
        };
    }

    #[actix_rt::test]
    async fn invalid_token_signature() {
        let database = get_database();
        let app_data = get_app_data();
        let user = User {
            id: random(),
            username: String::from("John Smith"),
            email: String::from("john_smith@example.com"),
            password_hash: PASSWORD_HASH.into(),
        };
        let token = Token::new_access_token(&"dsahsdadsh", &user.id, &UserAgent::Internal);
        database.add_user(&user).await.unwrap();
        let mut app =
            test::init_service(App::new().configure(|cfg| {
                crate::config(cfg, get_token_store(), database, app_data.clone())
            }))
            .await;

        let request = test::TestRequest::get()
            .uri("/whoami")
            .append_header((
                http::header::AUTHORIZATION,
                format!("Bearer {}", token.to_string()),
            ))
            .to_request();
        let response = test::call_service(&mut app, request).await;

        assert_eq!(
            response.status(),
            WhoamiError::InvalidToken(token::Error::VerifyError(
                token::VerifyError::InvalidSignature
            ))
            .status_code(),
            "unexpected status: {}, body: {:?}",
            response.status(),
            test::read_body(response).await
        );
        let response: WhoamiResponse = test::read_body_json(response).await;
        match response {
            Err(WhoamiError::InvalidToken(token::Error::VerifyError(
                token::VerifyError::InvalidSignature,
            ))) => (),
            _ => panic!("unexpected response: {:?}", response),
        };
    }
}
