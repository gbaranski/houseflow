pub mod clerk;
pub mod mailer;

pub mod controllers;
pub mod providers;

async fn health_check() -> &'static str {
    "I'm alive!"
}

pub fn app() -> axum::Router {
    use axum::routing::get;
    use axum::Router;

    Router::new()
        .route("/health-check", get(health_check))
        .nest(
            "/controllers",
            Router::new().nest("/meta", controllers::meta::app()),
        )
        .nest(
            "/providers",
            Router::new().nest("/lighthouse", providers::lighthouse::app()),
        )
}
