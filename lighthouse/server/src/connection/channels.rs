use super::{Request, Response};
use tokio::sync::{mpsc, watch};

/// Thread safe channel which allows sending Requests
pub type RequestSender = mpsc::Sender<Request>;

/// Not thread safe channel which allows reading Requests from RequestSender, this will be used
/// only internally by connection_write_loop()
pub type RequestReceiver = mpsc::Receiver<Request>;

/// Thread safe channel which allows retrieving Responses
pub type ResponseReceiver = watch::Receiver<Option<Response>>;

/// Not thread safe channel which allows sending Responses to ResponseReceiver, this will be used
/// only internally by connection_read_loop()
pub type ResponseSender = watch::Sender<Option<Response>>;

/// RequestResponseChannel combines:
/// - RequestSender: used to push new requests to stream
/// - ResponseReceiver: used to retrieve responsese from stream
pub type RequestResponseChannel = (ResponseReceiver, RequestSender);

