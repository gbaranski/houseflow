pub mod mailer;
pub mod clerk;

pub mod providers;
pub mod controllers;

async fn health_check() -> &'static str {
    "I'm alive!"
}

pub fn app() -> axum::Router {
    use axum::routing::get;

    axum::Router::new().route("/health-check", get(health_check))
}
