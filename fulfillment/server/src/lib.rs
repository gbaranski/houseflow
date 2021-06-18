use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
pub use config::Config;
use db::Database;
use lighthouse_api::prelude::Lighthouse;
use std::sync::Arc;
use types::{ServerSecrets, UserAgent};

pub mod config;
mod gactions;
mod internal;

#[derive(Clone)]
pub struct AgentData {
    user_agent: UserAgent,
}

pub(crate) fn config(
    cfg: &mut web::ServiceConfig,
    database: Data<dyn Database>,
    lighthouse: Data<dyn Lighthouse>,
    config: Config,
    secrets: ServerSecrets,
) {
    cfg.data(config)
        .data(secrets)
        .app_data(database)
        .app_data(lighthouse)
        .service(just_for_testing)
        .service(
            web::scope("/internal")
                .app_data(AgentData {
                    user_agent: UserAgent::Internal,
                })
                .service(internal::on_execute)
                .service(internal::on_sync),
        );
}

// TODO: remove that
#[actix_web::get("/just-for-testing")]
async fn just_for_testing(db: Data<dyn Database>) -> impl actix_web::Responder {
    use actix_web::HttpResponse;
    use semver::Version;
    use std::str::FromStr;
    use types::{Device, DevicePermission, DeviceType, UserID};

    let user_id = UserID::from_str("eeb3f58b28b8bd1815c3cc1bd0028fee").unwrap();

    let device = Device {
        id: rand::random(),
        password_hash: "$argon2i$v=19$m=4096,t=3,p=1$NjNjMTdhODU2YTJkNTdiZDViYjJkNTBhY2IxNmI4MzE$chXOPqhv21hnnp/C2Pv/UKm1tjSAXkBY3vkQzBNU9w8".to_string(),
        device_type: DeviceType::Gate,
        traits: vec![],
        name: "Gate".to_string(),
        will_push_state: true,
        room: None,
        model: "super-gate".to_string(),
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

pub async fn run(
    database: impl Database + 'static,
    lighthouse: impl Lighthouse + 'static,
    config: Config,
    secrets: ServerSecrets,
) -> std::io::Result<()> {
    let database = Data::from(Arc::new(database) as Arc<dyn Database>);
    let lighthouse = Data::from(Arc::new(lighthouse) as Arc<dyn Lighthouse>);

    log::info!("Starting `Auth` service");

    let address = format!("{}:{}", config.host, config.port);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(|cfg| {
                crate::config(
                    cfg,
                    database.clone(),
                    lighthouse.clone(),
                    config.clone(),
                    secrets.clone(),
                )
            })
    })
    .bind(address.clone())?;

    log::info!("Starting HTTP Server at `{}`", address);

    server.run().await?;

    Ok(())
}

#[cfg(test)]
mod test_utils {
    use db::memory::Database;

    use lighthouse_api::LighthouseMock;
    use lighthouse_proto::{execute, execute_response};
    use types::{Device, DeviceType, User, UserID};

    use actix_web::web::Data;
    use rand::RngCore;
    use std::sync::Arc;
    use tokio::sync::mpsc;

    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub fn get_config() -> crate::Config {
        crate::Config::default()
    }

    pub fn get_secrets() -> types::ServerSecrets {
        let gen_secret = || {
            let mut bytes = [0; 32];
            rand::thread_rng().fill_bytes(&mut bytes);
            hex::encode(bytes)
        };
        types::ServerSecrets {
            refresh_key: gen_secret(),
            access_key: gen_secret(),
            password_salt: gen_secret(),
        }
    }

    pub fn get_database() -> Data<dyn db::Database> {
        Data::from(Arc::new(Database::new()) as Arc<dyn db::Database>)
    }

    pub fn get_lighthouse() -> (
        Arc<lighthouse_api::LighthouseMock>,
        mpsc::Receiver<execute::Frame>,
        mpsc::Sender<execute_response::Frame>,
    ) {
        let (request_sender, request_receiver) = mpsc::channel(8);
        let (response_sender, response_receiver) = mpsc::channel(8);
        (
            Arc::new(LighthouseMock {
                request_sender,
                response_receiver: tokio::sync::Mutex::new(response_receiver),
            }),
            request_receiver,
            response_sender,
        )
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

    pub fn get_device() -> Device {
        use semver::Version;
        Device {
            id: rand::random(),
            password_hash: PASSWORD_HASH.into(),
            device_type: DeviceType::Gate,
            traits: vec![],
            name: String::from("SuperTestingGate"),
            will_push_state: true,
            room: Some(String::from("SuperTestingRoom")),
            model: String::from("gate-1200"),
            hw_version: Version::new(1, 0, 0),
            sw_version: Version::new(1, 0, 1),
            attributes: std::collections::HashMap::new(),
        }
    }
}
