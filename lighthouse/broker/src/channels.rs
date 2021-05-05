use lighthouse_api::{Request, Response};

use tokio::sync::{mpsc, broadcast};

const REQUEST_CHANNEL_BUFFER: usize = 32;
const RESPONSE_CHANNEL_BUFFER: usize = 32;

/// Thread safe channel which allows sending Requests
pub type RequestSender = mpsc::Sender<Request>;

/// Not thread safe channel which allows reading Requests from RequestSender, this will be used
/// only internally by connection_write_loop()
pub type RequestReceiver = mpsc::Receiver<Request>;

/// Thread safe channel which allows retrieving Responses
pub type ResponseReceiver = broadcast::Receiver<Response>;

/// Not thread safe channel which allows sending Responses to ResponseReceiver, this will be used
/// only internally by connection_read_loop()
pub type ResponseSender = broadcast::Sender<Response>;

/// RequestResponseChannel combines:
/// - RequestSender: used to push new requests to stream
/// - ResponseReceiver: used to retrieve responsese from stream
pub type RequestResponseChannel = (ResponseReceiver, RequestSender);

pub fn new_request_channel() -> (RequestReceiver, RequestSender) {
    let (tx, rx) = mpsc::channel::<Request>(REQUEST_CHANNEL_BUFFER);
    (rx, tx)
}

pub fn new_response_channel() -> (ResponseReceiver, ResponseSender) {
    let (tx, rx) = broadcast::channel::<Response>(RESPONSE_CHANNEL_BUFFER);
    (rx, tx)
}
