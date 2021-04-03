use futures::TryStreamExt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, StatusCode};

mod types;

#[derive(Serialize, Deserialize)]
pub enum Error {
    InvalidPath(String),
    InvalidDeviceID(String),
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Self::InvalidDeviceID(err.to_string())
    }
}



#[derive(Serialize, Deserialize)]
pub struct Response {
    pub error: Option<Error>,
}

async fn on_execute(device_id: Uuid) -> Result<hyper::Response<Body>, Error> {
    let body = Body::from("Received execute");
    let resp = hyper::Response::new(body);

    Ok(resp)
}

async fn on_query(device_id: Uuid) -> Result<hyper::Response<Body>, Error> {
    let body = Body::from("Received execute");
    let resp = hyper::Response::new(body);

    Ok(resp)
}

async fn handle(req: hyper::Request<Body>) -> Result<hyper::Response<Body>, Error> {
    // this is path without leading /
    let path: String = req
        .uri()
        .path()
        .chars()
        .skip(1)
        .collect(); 

    let mut splitted_path = path.splitn(2, "/");

    let (intent, device_id) = (
        splitted_path.next().ok_or_else(|| Error::InvalidPath("misssing intent".to_string()))?,
        splitted_path.next().ok_or_else(|| Error::InvalidPath("missing device_id".to_string()))?
    );

    let device_id = Uuid::parse_str(device_id)?;

    match intent {
        "execute" => on_execute(device_id).await,
        "query" => on_query(device_id).await,
        _ => Err(Error::InvalidPath("invalid intent".to_string())),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async {
        let service = service_fn(|req| async {
            Ok::<_, hyper::Error>(
                match handle(req).await {
                    Ok(resp) => resp,
                    Err(error) => {
                        let resp = Response {
                            error: Some(error),
                        };
                        let json = serde_json::to_string(&resp).unwrap();
                        let body = hyper::Body::from(json);
                        hyper::Response::new(body)
                    }
                })
        });

        Ok::<_, hyper::Error>(service)
    });

    let server = hyper::Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
