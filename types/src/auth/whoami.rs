use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub username: String,
    pub email: lettre::Address,
}
