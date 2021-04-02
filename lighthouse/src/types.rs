use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum ExecuteError {
    Timeout,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteRequest {
    pub params: std::collections::HashMap<String, String>,
    pub command: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteResponse {
    pub status: ExecuteResponseStatus,
    pub states: std::collections::HashMap<String, String>,
    pub error_code: Option<String>,
}


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecuteResponseStatus {
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
