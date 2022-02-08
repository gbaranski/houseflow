pub mod login;
pub mod refresh;
pub mod whoami;

pub fn app() -> axum::Router {
    use axum::routing::get;
    use axum::routing::post;

    axum::Router::new()
        .route(
            "/login",
            post(login::handle)
        )
        .route(
            "/refresh",
            post(refresh::handle)
        )
        .route(
            "/whoami",
            get(whoami::handle)
        )
}
