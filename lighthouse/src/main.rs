use futures::TryStreamExt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, StatusCode, Server};

mod types;

#[derive(Serialize, Deserialize)]
pub enum Error {
    MissingPathIntent,
    MissingPathDeviceID,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub error: Option<Error>,
}

async fn on_execute(device_id: Uuid) -> Result<hyper::Response<Body>, hyper::Error> {
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
        splitted_path.next().ok_or(Error::MissingPathIntent)?,
        splitted_path.next().ok_or(Error::MissingPathDeviceID)?
    );
    println!("Intent: {}, device id: {}", intent, device_id);

    Ok(hyper::Response::new(Body::from("Hello world!!")))
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
