use super::channels::RequestResponseChannel;
use houseflow_types::ClientID;
use lighthouse_api::{Request, RequestError, Response};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::sync::RwLock;

/// 5 seconds timeout for getting response for a request
const REQUEST_TIMEOUT_MILLIS: u64 = 5000;

/// Store holds thread safe RequestResponseChannels with corresponding ClientIDs
#[derive(Clone)]
pub struct Store {
    inner: Arc<RwLock<HashMap<ClientID, RequestResponseChannel>>>,
}

impl Store {
    /// Creates new store and returns it
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Used to add new Connection to store
    pub async fn add(&self, client_id: ClientID, channel: RequestResponseChannel) {
        self.inner.write().await.insert(client_id, channel);
    }

    /// Used to remove ClientID from store
    pub async fn remove(&self, client_id: &ClientID) {
        self.inner.write().await.remove(client_id);
    }

    /// Used to check if client with ID specified in argument exists
    pub async fn exists(&self, client_id: &ClientID) -> bool {
        self.inner.read().await.contains_key(client_id)
    }

    /// Sends request over RequestSender channel to connection with specific ClientID
    pub async fn send_request(
        &self,
        client_id: &ClientID,
        request: Request,
    ) -> Result<Response, RequestError> {
        let mut inner = self.inner.write().await;
        let (rx, tx) = inner
            .get_mut(client_id)
            .ok_or(RequestError::ClientNotConnected)?;

        let timeout = Duration::from_millis(REQUEST_TIMEOUT_MILLIS);

        tx.send(request).await.expect("receiver channel is closed");
        let response = tokio::time::timeout(timeout, rx.recv())
            .await
            .map_err(|_| RequestError::Timeout)?
            .expect("Sender half has been dropped");

        Ok(response)
    }
}
