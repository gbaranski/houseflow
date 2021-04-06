use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    DeviceNotFound,
    Timeout,
    IOError(String),
}

pub enum Request {
    Execute(ExecuteRequest),
    Query(QueryRequest),
}

impl Into<ExecuteRequest> for Request {
    fn into(self) -> ExecuteRequest {
        match self {
            Self::Execute(item) => item,
            _ => panic!("Request is not type of ExecuteRequest"),
        }
    }
}

impl Into<QueryRequest> for Request {
    fn into(self) -> QueryRequest {
        match self {
            Self::Query(item) => item,
            _ => panic!("Request is not type of QueryRequest"),
        }
    }
}

impl From<ExecuteRequest> for Request {
    fn from(item: ExecuteRequest) -> Self {
        Self::Execute(item)
    }
}

impl From<QueryRequest> for Request {
    fn from(item: QueryRequest) -> Self {
        Self::Query(item)
    }
}



pub enum Response {
    Execute(ExecuteResponse),
    Query(QueryResponse),
}


impl Into<ExecuteResponse> for Response {
    fn into(self) -> ExecuteResponse {
        match self {
            Self::Execute(item) => item,
            _ => panic!("Response is not type of ExecuteResponse"),
        }
    }
}

impl Into<QueryResponse> for Response {
    fn into(self) -> QueryResponse {
        match self {
            Self::Query(item) => item,
            _ => panic!("Response is not type of QueryResponse"),
        }
    }
}

impl From<ExecuteResponse> for Response {
    fn from(item: ExecuteResponse) -> Self {
        Self::Execute(item)
    }
}

impl From<QueryResponse> for Response {
    fn from(item: QueryResponse) -> Self {
        Self::Query(item)
    }
}



#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteRequest {
    pub params: HashMap<String, String>,
    pub command: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteResponse {
    pub status: ResponseStatus,
    pub states: HashMap<String, String>,
    pub error: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryRequest {}


#[derive(Serialize, Deserialize, Debug)]
pub struct QueryResponse {
    pub states: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
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
