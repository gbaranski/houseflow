use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use db::Database;
use std::sync::Arc;
use types::UserAgent;

mod gactions;
mod internal;

#[derive(Clone)]
pub struct AppData {
    pub refresh_key: Vec<u8>,
    pub access_key: Vec<u8>,
}

#[derive(Clone)]
pub struct AgentData {
    user_agent: UserAgent,
}

pub(crate) fn config(
    cfg: &mut web::ServiceConfig,
    database: Data<dyn Database>,
    app_data: AppData,
) {
    cfg.data(app_data)
        .app_data(database)
        .service(
            web::scope("/internal")
                .app_data(AgentData {
                    user_agent: UserAgent::Internal,
                })
                .service(internal::on_sync),
        );
}

pub async fn run(
    address: impl std::net::ToSocketAddrs + std::fmt::Display + Clone,
    database: impl Database + 'static,
    app_data: AppData,
) -> std::io::Result<()> {
    let database = Data::from(Arc::new(database) as Arc<dyn Database>);

    log::info!("Starting `Auth` service");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .configure(|cfg| {
                config(
                    cfg,
                    database.clone(),
                    app_data.clone(),
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
    use super::Database;
    use db::MemoryDatabase;
    use types::{User, UserID, Device, DeviceType};

    use actix_web::web::Data;
    use rand::RngCore;
    use std::sync::Arc;

    pub const PASSWORD_HASH: &str = "$argon2i$v=19$m=4096,t=3,p=1$Zcm15qxfZSBqL9K6S9G5mNIGgz7qmna7TlPPN+t9mqA$ECoZv8pF6Ew6gjh8b9d2oe4QtQA3DO5PIfuWvK2h3OU";

    pub fn get_app_data() -> crate::AppData {
        let mut app_data = crate::AppData {
            refresh_key: vec![0; 32],
            access_key: vec![0; 32],
        };
        rand::thread_rng().fill_bytes(&mut app_data.refresh_key);
        rand::thread_rng().fill_bytes(&mut app_data.access_key);
        app_data
    }

    pub fn get_database() -> Data<dyn Database> {
        Data::from(Arc::new(MemoryDatabase::new()) as Arc<dyn Database>)
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
            name:  String::from("SuperTestingGate"),
            will_push_state: true,
            room: Some(String::from("SuperTestingRoom")),
            model: String::from("gate-1200"),
            hw_version: Version::new(1, 0, 0),
            sw_version: Version::new(1, 0, 1),
            attributes: std::collections::HashMap::new(),
        }
    }

}
