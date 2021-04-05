use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::pin::Pin;
use std::boxed::Box;
use std::future::Future;
use std::task::{Context, Poll};
use std::sync::RwLock;
use std::sync::Mutex;
use std::collections::HashMap;
use warp::{http, Filter};
use super::Error;
use super::DeviceSessions;

#[derive(Clone)]
struct Store {
    device_sessions: Arc<RwLock<DeviceSessions>>,
    counter: Arc<RwLock<usize>>
}

impl Store {
    fn new() -> Self {
        Self {
            device_sessions: Arc::new(RwLock::new(HashMap::new())),
            counter: Arc::new(RwLock::new(0)),
        }
    }
}

pub async fn serve() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let execute_path = warp::post()
        .and(warp::path("execute"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(on_execute);

    warp::serve(execute_path)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn on_execute(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    *store.counter.write().unwrap() += 1;

    Ok(warp::reply::with_status(
            format!("Received execute intent for deviceID: {}, counter: {}", id, store.counter.read().unwrap()),
            http::StatusCode::OK
            ))

}


// async fn handle(req: HttpRequest<HttpBody>) -> Result<HttpResponse<HttpBody>, Error> {
//     // this is path without leading /
//     let path: String = req
//         .uri()
//         .path()
//         .chars()
//         .skip(1)
//         .collect(); 

//     let mut splitted_path = path.splitn(2, "/");

//     let (intent, device_id) = (
//         splitted_path.next().ok_or_else(|| Error::InvalidPath("misssing intent".to_string()))?,
//         splitted_path.next().ok_or_else(|| Error::InvalidPath("missing device_id".to_string()))?
//         );

//     let device_id = Uuid::parse_str(device_id)?;

//     match intent {
//         "execute" => Self::on_execute(device_id).await,
//         "query" => Self::on_query(device_id).await,
//         _ => Err(Error::InvalidPath("invalid intent".to_string())),
//     }
// }
