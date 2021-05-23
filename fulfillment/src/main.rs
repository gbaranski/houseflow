use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
}


#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    log::info!("Starting houseflow-fulfillment");

    let db = Database::connect()?;
    db.init().await?;

    let mc = memcache::connect("memcache://memcache:11211?timeout=10&tcp_nodelay=true")?;
    
    let app_state = AppState {
        db,
        mc,
    };

    log::info!("Starting HttpServer");
    HttpServer::new(move || {
        App::new()
            .data(app_state.to_owned())
            .wrap(actix_web::middleware::Logger::default())
            .service(webhook)
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
    .unwrap();

    Ok(())
}


