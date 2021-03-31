use std::time::Duration;
use tokio::sync::oneshot;
use uuid::Uuid;
use super::session::WebsocketSession;
use std::boxed::Box;
use actix::Handler;
use crate::ExecuteError as Error;
use crate::ExecuteRequest as Request;
use crate::ExecuteResponseFuture as ResponseFuture;

const EXECUTE_TIMEOUT: Duration = Duration::from_secs(5);



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
                Err(_) => Err(Error::Timeout),
            }
        });

        ResponseFuture(Box::pin(boxed))
    }
}
