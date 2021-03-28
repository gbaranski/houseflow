use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct ExecuteRequest {
    pub params: std::collections::HashMap<String, String>,
    pub command: String,
    pub device_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: PayloadCommandStatus,
    pub state: std::collections::HashMap<String, String>,
    pub error_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum PayloadCommandStatus {
    /// Confirm that the command succeeded.
    #[serde(rename = "SUCCESS")]
    Success,

    /// Command is enqueued but expected to succeed.
    #[serde(rename = "PENDING")]
    Pending,

    /// Target device is in offline state or unreachable.
    #[serde(rename = "OFFLINE")]
    Offline,

    /// There is an issue or alert associated with a command. 
    /// The command could succeed or fail. 
    /// This status type is typically set when you want to send additional information about another connected device.
    #[serde(rename = "EXCEPTIONS")]
    Exceptions,

    /// Target device is unable to perform the command.
    #[serde(rename = "ERROR")]
    Error
}

#[derive(Serialize, Deserialize)]
pub struct QueryRequest {
    pub device_id: Uuid,
}
