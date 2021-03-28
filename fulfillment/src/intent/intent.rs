use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use crate::intent;


#[derive(Deserialize)]
#[serde(tag = "intent", content = "payload")]
pub enum RequestPayload {
    #[serde(rename = "action.devices.EXECUTE")]
    Execute(intent::execute::request::Payload),

    // #[serde(rename = "action.devices.QUERY")]
    // Query(inte),
}


#[derive(Deserialize)]
pub struct RequestInput {
    /// Intent request type. Constant for each intent
    pub intent: String,
    /// Request payload
    pub payload: Option<RequestPayload>
}

#[derive(Deserialize)]
pub struct Request {
    /// ID of the request.
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// List of inputs matching the intent request.
    pub inputs: Vec<RequestInput>
}


/// Use it only for returning errors before redirecting intent request to specific handler
#[derive(Serialize)]
pub struct BaseResponsePayload {
    /// Reflects the unique (and immutable) user ID on the agent's platform.
    #[serde(rename = "agentUserId")]
    pub user_id: Uuid,

    /// For systematic errors on SYNC
    #[serde(rename = "errorCode")]
    pub error_code: String,

    /// Detailed error which will never be presented to users but may be logged or used during development.
    #[serde(rename = "debugString")]
    pub debug_string: String,
}

/// Use it only for returning errors before redirecting intent request to specific handler
#[derive(Serialize)]
pub struct BaseResponse {
    /// ID of the request.
    #[serde(rename = "requestId")]
    pub request_id: String,

    /// Intent response payload.
    pub payload: BaseResponsePayload,
}
