#[macro_use]
macro_rules! admin_method {
    ($name: ident, $base_mod: path, $database_add: expr, $from_request: expr, $to_response: expr) => {
        pub mod $name {
            use super::*;
            use actix_web::{
                web::{Data, Json},
                HttpRequest,
            };
            use houseflow_config::server::Config;
            use houseflow_db::Database;
            use houseflow_types::token::AccessToken;
            use $base_mod as base;

            pub async fn on_add(
                Json(request): Json<base::add::Request>,
                http_request: HttpRequest,
                config: Data<Config>,
                db: Data<dyn Database>,
            ) -> Result<Json<base::add::ResponseBody>, base::add::ResponseError> {
                let access_token =
                    AccessToken::from_request(config.secrets.access_key.as_bytes(), &http_request)?;
                if !db
                    .check_user_admin(&access_token.sub)
                    .map_err(houseflow_db::Error::into_internal_server_error)?
                {
                    return Err(base::add::ResponseError::UserNotAdmin);
                }
                let item = $from_request(request);

                $database_add(db.as_ref(), &item)
                    .map_err(houseflow_db::Error::into_internal_server_error)?;

                Ok(Json($to_response(item)))
            }
        }
    };
}

use houseflow_types::{admin, Device, Room, Structure, UserStructure};

admin_method!(
    structure,
    admin::structure,
    Database::add_structure,
    (|req: admin::structure::add::Request| {
        Structure {
            id: rand::random(),
            name: req.structure_name,
        }
    }),
    (|item: Structure| {
        admin::structure::add::ResponseBody {
            structure_id: item.id,
        }
    })
);

admin_method!(
    room,
    admin::room,
    Database::add_room,
    (|req: admin::room::add::Request| {
        Room {
            id: rand::random(),
            structure_id: req.structure_id,
            name: req.room_name,
        }
    }),
    (|item: Room| { admin::room::add::ResponseBody { room_id: item.id } })
);

admin_method!(
    device,
    admin::device,
    Database::add_device,
    (|req: admin::device::add::Request| {
        Device {
            id: rand::random(),
            room_id: req.room_id,
            password_hash: Some(
                argon2::hash_encoded(
                    req.password.as_bytes(),
                    &crate::get_password_salt(),
                    &argon2::Config::default(),
                )
                .unwrap(),
            ),
            device_type: req.device_type,
            traits: req.traits,
            name: req.name,
            will_push_state: req.will_push_state,
            model: req.model,
            hw_version: req.hw_version,
            sw_version: req.sw_version,
            attributes: req.attributes,
        }
    }),
    (|item: Device| { admin::device::add::ResponseBody { device_id: item.id } })
);

admin_method!(
    user_structure,
    admin::user_structure,
    Database::add_user_structure,
    (|req: admin::user_structure::add::Request| {
        UserStructure {
            structure_id: req.structure_id,
            user_id: req.user_id,
            is_manager: req.is_manager,
        }
    }),
    (|_item: UserStructure| { admin::user_structure::add::ResponseBody {} })
);
