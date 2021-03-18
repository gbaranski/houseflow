use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "intent", content = "payload")]
pub enum RequestPayload {
    #[serde(rename = "action.devices.EXECUTE")]
    Execute(crate::intent::ExecutePayload),

    #[serde(rename = "action.devices.QUERY")]
    Query(crate::intent::QueryPayload),
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
