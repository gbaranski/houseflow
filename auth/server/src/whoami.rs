use actix_web::{
    get, http,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_auth_types::{WhoamiError, WhoamiResponse, WhoamiResponseBody};
use houseflow_db::Database;
use houseflow_token::Token;

#[get("/whoami")]
pub async fn whoami(
    req: HttpRequest,
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
    let token = Token::from_str(token)?;
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
