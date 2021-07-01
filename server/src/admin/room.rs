use actix_web::{
    put,
    web::{Data, Json},
    HttpRequest,
};
use houseflow_config::server::Secrets;
use houseflow_db::Database;
use houseflow_types::UserAgent;
use houseflow_types::{
    admin::{AddRoomRequest, AddRoomResponse, AddRoomResponseBody, AddRoomResponseError},
    token::Token,
    Room,
};

const USER_AGENT: UserAgent = UserAgent::Internal;

#[put("/room")]
pub async fn on_add_room(
    add_room_request: Json<AddRoomRequest>,
    http_request: HttpRequest,
    secrets: Data<Secrets>,
    db: Data<dyn Database>,
) -> Result<Json<AddRoomResponse>, AddRoomResponseError> {
    let add_room_request = add_room_request.0;
    let access_token = Token::from_request(&http_request)?;
    access_token.verify(&secrets.access_key, Some(&USER_AGENT))?;

    if !db
        .check_user_admin(access_token.user_id())
        .await
        .map_err(|err| AddRoomResponseError::InternalError(err.to_string()))?
    {
        return Err(AddRoomResponseError::UserNotAdmin);
    }

    let room = Room {
        id: rand::random(),
        structure_id: add_room_request.structure_id,
        name: add_room_request.room_name,
    };

    db.add_room(&room)
        .await
        .map_err(|err| AddRoomResponseError::InternalError(err.to_string()))?;

    let response = AddRoomResponseBody { room_id: room.id };

    Ok(Json(AddRoomResponse::Ok(response)))
}
