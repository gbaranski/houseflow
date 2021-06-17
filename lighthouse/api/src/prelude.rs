use crate::Error;
use async_trait::async_trait;
use lighthouse_proto::{execute, execute_response};
use types::DeviceID;

#[async_trait]
pub trait Lighthouse: Send + Sync {
    async fn execute(
        &self,
        frame: &execute::Frame,
        device_id: &DeviceID,
    ) -> Result<execute_response::Frame, Error>;
}
