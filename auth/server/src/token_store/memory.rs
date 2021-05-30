use super::{TokenStore, TokenStoreInternalError};
use async_trait::async_trait;
use houseflow_token::{Token, TokenID};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl TokenStoreInternalError for Error {}

#[derive(Clone)]
pub struct MemoryTokenStore {
    store: Arc<Mutex<HashMap<TokenID, Token>>>,
}

impl MemoryTokenStore {
    pub fn new() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

#[async_trait]
impl TokenStore for MemoryTokenStore {
    async fn exists(&self, id: &TokenID) -> Result<bool, super::Error> {
        Ok(self.store.lock().unwrap().contains_key(id))
    }

    async fn set(&self, token: &Token) -> Result<(), super::Error> {
        self.store
            .lock()
            .unwrap()
            .insert(token.payload.id.clone(), token.clone());
        Ok(())
    }
}
