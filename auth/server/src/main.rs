use houseflow_auth_types::{AccessTokenRequest, AccessTokenResponse};
use warp::{
    http::{Response, StatusCode},
    Filter, Reply, reply::Json,
};

async fn on_token(req: AccessTokenRequest) -> Json {
    todo!()
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let token = warp::post()
        .and(warp::path("token"))
        .and(warp::query::query::<AccessTokenRequest>())
        .map(on_token);
    warp::serve(token).run(([127, 0, 0, 1], 3030)).await;
    println!("Hello world");
}
