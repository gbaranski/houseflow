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
    pub status: ResponseStatus,
    pub states: std::collections::HashMap<String, String>,
    pub error_code: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseStatus {
    /// Confirm that the command succeeded.
    Success,

    /// Command is enqueued but expected to succeed.
    Pending,

    /// Target device is in offline state or unreachable.
    Offline,

    /// There is an issue or alert associated with a command. 
    /// The command could succeed or fail. 
    /// This status type is typically set when you want to send additional information about another connected device.
    Exceptions,

    /// Target device is unable to perform the command.
    Error
}

#[derive(Serialize, Deserialize)]
pub struct QueryRequest {
    pub device_id: Uuid,
}
