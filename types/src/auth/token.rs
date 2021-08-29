use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Request {}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Response {
    pub refresh_token: Option<String>,

    pub access_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, thiserror::Error)]
pub enum Error {}
