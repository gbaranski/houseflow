use super::{TokenStore, TokenStoreInternalError};
use async_trait::async_trait;
use houseflow_types::token::{Token, TokenID};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl TokenStoreInternalError for Error {}

#[derive(Clone, Default)]
pub struct MemoryTokenStore {
    store: Arc<Mutex<HashMap<TokenID, Token>>>,
}

impl MemoryTokenStore {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait]
impl TokenStore for MemoryTokenStore {
    async fn exists(&self, id: &TokenID) -> Result<bool, super::Error> {
        Ok(self.store.lock().await.contains_key(id))
    }

    async fn get(&self, id: &TokenID) -> Result<Option<Token>, super::Error> {
        Ok(self.store.lock().await.get(id).cloned())
    }

    async fn add(&self, token: &Token) -> Result<(), super::Error> {
        self.store
            .lock()
            .await
            .insert(token.id().clone(), token.clone());
        Ok(())
    }

    async fn remove(&self, id: &TokenID) -> Result<bool, super::Error> {
        Ok(self.store.lock().await.remove(id).is_some())
    }
}
