use actix_web::{
    put,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Secrets;
use houseflow_db::Database;
use houseflow_types::UserAgent;
use houseflow_types::{
    admin::{
        AddUserStructureRequest, AddUserStructureResponse, AddUserStructureResponseBody,
        AddUserStructureResponseError,
    },
    token::Token,
    UserStructure,
};

const USER_AGENT: UserAgent = UserAgent::Internal;

#[put("/user_structure")]
pub async fn on_add_user_structure(
    add_user_structure_request: Json<AddUserStructureRequest>,
    http_request: HttpRequest,
    secrets: Data<Secrets>,
    db: Data<dyn Database>,
) -> Result<Json<AddUserStructureResponse>, AddUserStructureResponseError> {
    let add_user_structure_request = add_user_structure_request.0;
    let access_token = Token::from_request(&http_request)?;
    access_token.verify(&secrets.access_key, Some(&USER_AGENT))?;

    if !db
        .check_user_admin(access_token.user_id())
        .await
        .map_err(|err| AddUserStructureResponseError::InternalError(err.to_string()))?
    {
        return Err(AddUserStructureResponseError::UserNotAdmin);
    }

    let user_structure = UserStructure {
        structure_id: add_user_structure_request.structure_id,
        user_id: add_user_structure_request.user_id,
        is_manager: add_user_structure_request.is_manager,
    };

    db.add_user_structure(&user_structure)
        .await
        .map_err(|err| AddUserStructureResponseError::InternalError(err.to_string()))?;

    let response = AddUserStructureResponseBody {};

    Ok(Json(AddUserStructureResponse::Ok(response)))
}
