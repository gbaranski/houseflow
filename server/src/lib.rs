mod auth;
mod fulfillment;
mod lighthouse;
mod token_store;

pub use token_store::{MemoryTokenStore, RedisTokenStore, TokenStore};

use actix_web::web;
use houseflow_config::server::Secrets;
use houseflow_db::Database;

use {
    houseflow_types::DeviceID, lighthouse::Session, std::collections::HashMap, tokio::sync::Mutex,
};
pub type Sessions = Mutex<HashMap<DeviceID, actix::Addr<Session>>>;

pub fn configure(
    cfg: &mut web::ServiceConfig,
    token_store: web::Data<dyn TokenStore>,
    database: web::Data<dyn Database>,
    secrets: web::Data<Secrets>,
    sessions: web::Data<Sessions>,
) {
    cfg.app_data(secrets)
        .app_data(token_store)
        .app_data(sessions)
        .app_data(database)
        .service(
            web::scope("/auth")
                .service(auth::on_login)
                .service(auth::on_logout)
                .service(auth::on_register)
                .service(auth::on_whoami)
                .service(
                    web::scope("/")
                        .app_data(auth::on_exchange_refresh_token_form_config())
                        .service(auth::on_exchange_refresh_token),
                ),
        )
        .service(
            web::scope("/fulfillment").service(
                web::scope("/internal")
                    .service(fulfillment::internal::on_execute)
                    .service(fulfillment::internal::on_query)
                    .service(fulfillment::internal::on_sync),
            ),
        )
        .service(web::scope("/lighthouse").service(lighthouse::on_websocket))
        .service(just_for_testing);
}

// TODO: remove that
#[actix_web::get("/just-for-testing")]
async fn just_for_testing(db: web::Data<dyn Database>) -> impl actix_web::Responder {
    use actix_web::HttpResponse;
    use houseflow_types::{Device, DeviceTrait, DeviceType, UserID, Structure, Room, UserStructure};
    use semver::Version;
    use std::str::FromStr;

    let user_id = UserID::from_str("eeb3f58b28b8bd1815c3cc1bd0028fee").unwrap();
    
    let structure = Structure {
        id: rand::random(),
        name: "zukowo".to_string(),
    };

    let room = Room {
        id: rand::random(),
        name: "garage".to_string(),
        structure_id: structure.id.clone(),
    };

    let user_structure = UserStructure {
        structure_id: structure.id.clone(),
        user_id,
        is_manager: true,
    };

    let device = Device {
        id: rand::random(),
        room_id: room.id.clone(),
        password_hash: "$argon2i$v=19$m=4096,t=3,p=1$NjNjMTdhODU2YTJkNTdiZDViYjJkNTBhY2IxNmI4MzE$chXOPqhv21hnnp/C2Pv/UKm1tjSAXkBY3vkQzBNU9w8".to_string(),
        device_type: DeviceType::Light,
        traits: vec![DeviceTrait::OnOff],
        name: "Night Lamp".to_string(),
        will_push_state: true,
        model: "super-lamp".to_string(),
        hw_version: Version::parse("0.1.0").unwrap(),
        sw_version: Version::parse("0.1.0").unwrap(),
        attributes: Default::default(),
    };
    db.add_structure(&structure).await.unwrap();
    db.add_room(&room).await.unwrap();
    db.add_device(&device).await.unwrap();
    db.add_user_structure(&user_structure).await.unwrap();

    HttpResponse::Ok()
}

#[cfg(test)]
mod test_utils {
    use crate::{MemoryTokenStore, TokenStore};
    use houseflow_db::memory::Database;
    use houseflow_types::{Device, DeviceType, User, UserID, Room, Structure};

    use actix_web::web::Data;
    use std::sync::Arc;

    pub const PASSWORD: &str = "SomePassword";
    pub const PASSWORD_INVALID: &str = "SomeOtherPassword";
    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub fn get_database() -> Data<dyn houseflow_db::Database> {
        Data::from(Arc::new(Database::new()) as Arc<dyn houseflow_db::Database>)
    }

    pub fn get_token_store() -> Data<dyn TokenStore> {
        Data::from(Arc::new(MemoryTokenStore::new()) as Arc<dyn TokenStore>)
    }

    pub fn get_user() -> User {
        let id: UserID = rand::random();
        User {
            id: id.clone(),
            username: format!("john-{}", id.clone()),
            email: format!("john-{}@example.com", id.clone()),
            password_hash: PASSWORD_HASH.into(),
        }
    }

    pub fn get_structure() -> Structure {
        Structure {
            id: rand::random(),
            name: "test-home".to_string(),
        }
    }


    pub fn get_room(structure: &Structure) -> Room {
        Room {
            id: rand::random(),
            structure_id: structure.id.clone(),
            name: "test-garage".to_string(),
        }
    }

    pub fn get_device(room: &Room) -> Device {
        use semver::Version;

        Device {
            id: rand::random(),
            room_id: room.id.clone(),
            password_hash: PASSWORD_HASH.into(),
            device_type: DeviceType::Gate,
            traits: vec![],
            name: String::from("SuperTestingGate"),
            will_push_state: true,
            model: String::from("gate-1200"),
            hw_version: Version::new(1, 0, 0),
            sw_version: Version::new(1, 0, 1),
            attributes: std::collections::HashMap::new(),
        }
    }

}
