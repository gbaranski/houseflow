use serde::{ Serialize, Deserialize };
use crate::intent;


#[derive(Deserialize)]
#[serde(tag = "intent", content = "payload")]
pub enum RequestPayload {
    #[serde(rename = "action.devices.EXECUTE")]
    Execute(intent::execute::request::Payload),

    #[serde(rename = "action.devices.SYNC")]
    Sync(),

    // #[serde(rename = "action.devices.QUERY")]
    // Query(intent::query::),
}


#[derive(Deserialize)]
pub struct RequestInput {
    /// Intent request type. Constant for each intent
    pub intent: String,
    /// Request payload
    pub payload: RequestPayload
}

#[derive(Deserialize)]
pub struct Request {
    /// ID of the request.
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// List of inputs matching the intent request.
    pub inputs: Vec<RequestInput>
}

#[derive(Serialize)]
pub enum ResponsePayload {
    Execute(intent::execute::response::Payload),
    Sync(intent::sync::response::Payload),
    // Disconnect()
}


#[derive(Serialize)]
pub struct Response {
    /// ID of the response.
    #[serde(rename = "requestId")]
    pub request_id: String,

    /// Intent response payload.
    pub payload: ResponsePayload,
}
