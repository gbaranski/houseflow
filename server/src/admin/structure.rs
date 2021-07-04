use actix_web::{
    put,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Config;
use houseflow_db::Database;
use houseflow_types::{
    admin::structure::add::{Request, ResponseBody, ResponseError},
    token::AccessToken,
    Structure,
};

#[put("/structure")]
pub async fn on_add(
    Json(request): Json<Request>,
    http_request: HttpRequest,
    config: Data<Config>,
    db: Data<dyn Database>,
) -> Result<Json<ResponseBody>, ResponseError> {
    let access_token = AccessToken::from_request(config.secrets.access_key.as_bytes(), &http_request)?;

    if !db
        .check_user_admin(&access_token.sub)
        .await
        .map_err(houseflow_db::Error::into_internal_server_error)?
    {
        return Err(ResponseError::UserNotAdmin);
    }

    let structure = Structure {
        id: rand::random(),
        name: request.structure_name,
    };

    db.add_structure(&structure)
        .await
        .map_err(houseflow_db::Error::into_internal_server_error)?;

    Ok(Json(ResponseBody {
        structure_id: structure.id,
    }))
}
