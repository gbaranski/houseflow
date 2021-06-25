mod auth;
mod fulfillment;
mod lighthouse;

use actix_web::web;
use config::server::Secrets;
use db::Database;

use {lighthouse::Session, std::collections::HashMap, tokio::sync::Mutex, types::DeviceID};
pub type Sessions = Mutex<HashMap<DeviceID, actix::Addr<Session>>>;

pub fn configure(
    cfg: &mut web::ServiceConfig,
    token_store: web::Data<dyn token::store::TokenStore>,
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
    use semver::Version;
    use std::str::FromStr;
    use types::{Device, DevicePermission, DeviceTrait, DeviceType, UserID};

    let user_id = UserID::from_str("eeb3f58b28b8bd1815c3cc1bd0028fee").unwrap();

    let device = Device {
        id: rand::random(),
        password_hash: "$argon2i$v=19$m=4096,t=3,p=1$NjNjMTdhODU2YTJkNTdiZDViYjJkNTBhY2IxNmI4MzE$chXOPqhv21hnnp/C2Pv/UKm1tjSAXkBY3vkQzBNU9w8".to_string(),
        device_type: DeviceType::Light,
        traits: vec![DeviceTrait::OnOff],
        name: "Night Lamp".to_string(),
        will_push_state: true,
        room: None,
        model: "super-lamp".to_string(),
        hw_version: Version::parse("0.1.0").unwrap(),
        sw_version: Version::parse("0.1.0").unwrap(),
        attributes: Default::default(),
    };
    db.add_device(&device).await.unwrap();
    db.add_user_device(
        &device.id,
        &user_id,
        &DevicePermission {
            read: true,
            write: true,
            execute: true,
        },
    )
    .await
    .unwrap();

    HttpResponse::Ok()
}
