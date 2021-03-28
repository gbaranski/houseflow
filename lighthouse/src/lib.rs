use uuid::Uuid;
use memcache::MemcacheError;
use serde::{Deserialize, Serialize};

mod types;
pub use types::*;

#[derive(Serialize, Deserialize)]
#[serde(tag = "error_code")]
pub enum Error {
    MemcacheError(String),
    ReqwestError(String),
}


impl From<MemcacheError> for Error {
    fn from(err: MemcacheError) -> Self {
        Error::MemcacheError(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err.to_string())
    }
}

#[derive(Clone)]
pub struct LighthouseAPI<'a> {
    memcache: &'a memcache::Client,
    db: &'a houseflow_db::Database,
}


impl<'a> LighthouseAPI<'a> {
    pub fn get_wealthy_lighthouse_address(
        &self, 
        device_id: &Uuid
    ) -> Result<Option<String>, Error> {
        Ok(
            self.memcache.get(&device_id.to_string())?
        )
    }

    pub async fn send_execute(
        &self, 
        lighthouse_address: String,
        req: ExecuteRequest,
    ) -> Result<Response, Error> {
        let url = format!("http://{}/execute", lighthouse_address);

        reqwest::Client::new()
            .post(url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?
    }

    pub async fn send_query(
        &self, 
        lighthouse_address: String,
        req: QueryRequest,
    ) -> Result<Response, Error> {
        let url = format!("http://{}/query/{}", lighthouse_address, req.device_id);

        reqwest::Client::new()
            .get(url)
            .json(&req)
            .send()
            .await?
            .json()
            .await?
    }
}
