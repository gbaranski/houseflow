use lighthouse_proto::{execute, execute_response};
use crate::Error;
use types::DeviceID;
use async_trait::async_trait;

#[async_trait]
pub trait Lighthouse: Send + Sync {
    async fn execute(
        &self,
        frame: &execute::Frame,
        device_id: &DeviceID,
    ) -> Result<execute_response::Frame, Error>;
}
