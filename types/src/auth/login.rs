use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct Request {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Response {
    pub refresh_token: String,

    pub access_token: String,
}
