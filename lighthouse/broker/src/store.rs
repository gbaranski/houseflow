use super::channels::RequestResponseChannel;
use houseflow_types::DeviceID;
use lighthouse_api::{Request, RequestError, Response};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::RwLock;

/// 5 seconds timeout for getting response for a request
const REQUEST_TIMEOUT_MILLIS: u64 = 5000;

/// Store holds thread safe RequestResponseChannels with corresponding DeviceIDs
#[derive(Clone)]
pub struct Store {
    inner: Arc<RwLock<HashMap<DeviceID, RequestResponseChannel>>>,
}

impl Store {
    /// Creates new store and returns it
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Used to add new Connection to store
    pub async fn add(&self, device_id: DeviceID, channel: RequestResponseChannel) {
        self.inner.write().await.insert(device_id, channel);
    }

    /// Used to remove DeviceID from store
    pub async fn remove(&self, device_id: &DeviceID) {
        self.inner.write().await.remove(device_id);
    }

    /// Used to check if device with ID specified in argument exists
    pub async fn exists(&self, device_id: &DeviceID) -> bool {
        self.inner.read().await.contains_key(device_id)
    }

    /// Sends request over RequestSender channel to connection with specific DeviceID
    pub async fn send_request(
        &self,
        device_id: &DeviceID,
        request: Request,
    ) -> Result<Response, RequestError> {
        let mut inner = self.inner.write().await;
        let (rx, tx) = inner
            .get_mut(device_id)
            .ok_or(RequestError::DeviceNotConnected)?;

        let timeout = Duration::from_millis(REQUEST_TIMEOUT_MILLIS);

        tx.send(request).await.expect("receiver channel is closed");
        let response = tokio::time::timeout(timeout, rx.recv())
            .await
            .map_err(|_| RequestError::Timeout)?
            .expect("Sender half has been dropped");

        Ok(response)
    }
}
