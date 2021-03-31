use std::time::Duration;
use actix::prelude::*;
use tokio::sync::oneshot;
use uuid::Uuid;
use actix::dev::*;
use super::session::WebsocketSession;

const EXECUTE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub enum ExecuteError {
    Timeout,
}


#[derive(Debug, Clone)]
pub struct Response(pub String);

use std::future::Future;
use std::pin::Pin;
use std::boxed::Box;
use std::task::Poll;


#[derive(MessageResponse)]
pub struct ResponseFuture(pub Pin<Box<dyn Future<Output = Result<Response, ExecuteError>> + Send>>);

// impl Future for ResponseFuture {
//     type Output = Response;

//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
//         self.0.
//     }
// }

#[derive(Message, Debug)]
#[rtype(result = "ResponseFuture")]
pub struct Request(pub String);


impl Handler<Request> for WebsocketSession {

    type Result = ResponseFuture;

    fn handle(
        &mut self, 
        _req: Request, 
        ctx: &mut Self::Context
    ) -> Self::Result {
        ctx.text("Send me response");
        let (tx, rx) = oneshot::channel();

        let request_id = Uuid::new_v4();
        self.response_channels.push((request_id, Some(tx)));

        let boxed = Box::pin(async move {

            match tokio::time::timeout(EXECUTE_TIMEOUT, rx).await {
                Ok(resp) => Ok(resp.unwrap()),
                Err(_) => Err(ExecuteError::Timeout),
            }
        });

        ResponseFuture(Box::pin(boxed))
    }
}
