use super::{TokenStore, TokenStoreInternalError};
use async_trait::async_trait;
use houseflow_types::token::RefreshTokenID;
use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl TokenStoreInternalError for Error {}

#[derive(Clone, Default)]
pub struct MemoryTokenStore {
    store: Arc<Mutex<HashSet<RefreshTokenID>>>,
}

impl MemoryTokenStore {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl TokenStore for MemoryTokenStore {
    async fn exists(&self, id: &RefreshTokenID) -> Result<bool, super::Error> {
        Ok(self.store.lock().await.contains(id))
    }

    async fn add(&self, id: &RefreshTokenID) -> Result<(), super::Error> {
        self.store.lock().await.insert(id.clone());

        Ok(())
    }

    async fn remove(&self, id: &RefreshTokenID) -> Result<bool, super::Error> {
        Ok(self.store.lock().await.remove(id))
    }
}
