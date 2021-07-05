use actix_web::{
    put,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    admin::user_structure::add::{Request, ResponseBody, ResponseError},
    token::AccessToken,
    UserStructure,
};

#[put("/user_structure")]
pub async fn on_add(
    Json(request): Json<Request>,
    http_request: HttpRequest,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<Json<ResponseBody>, ResponseError> {
    let access_token =
        AccessToken::from_request(config.secrets.access_key.as_bytes(), &http_request)?;

    if !db
        .check_user_admin(&access_token.sub)
        .map_err(houseflow_db::Error::into_internal_server_error)?
    {
        return Err(ResponseError::UserNotAdmin);
    }

    let user_structure = UserStructure {
        structure_id: request.structure_id,
        user_id: request.user_id,
        is_manager: request.is_manager,
    };

    db.add_user_structure(&user_structure)
        .map_err(houseflow_db::Error::into_internal_server_error)?;

    Ok(Json(ResponseBody {}))
}
