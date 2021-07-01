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
        AddStructureRequest, AddStructureResponse, AddStructureResponseBody,
        AddStructureResponseError,
    },
    token::Token,
    Structure,
};

#[put("/structure")]
pub async fn on_add_structure(
    add_structure_request: Json<AddStructureRequest>,
    http_request: HttpRequest,
    secrets: Data<Secrets>,
    db: Data<dyn Database>,
) -> Result<Json<AddStructureResponse>, AddStructureResponseError> {
    let add_structure_request = add_structure_request.0;
    let access_token = Token::from_request(&http_request)?;
    access_token.verify(&secrets.access_key, Some(&UserAgent::Internal))?;

    if !db
        .check_user_admin(access_token.user_id())
        .await
        .map_err(|err| AddStructureResponseError::InternalError(err.to_string()))?
    {
        return Err(AddStructureResponseError::UserNotAdmin);
    }

    let structure = Structure {
        id: rand::random(),
        name: add_structure_request.structure_name,
    };

    db.add_structure(&structure)
        .await
        .map_err(|err| AddStructureResponseError::InternalError(err.to_string()))?;

    let response = AddStructureResponseBody {
        structure_id: structure.id,
    };

    Ok(Json(AddStructureResponse::Ok(response)))
}
