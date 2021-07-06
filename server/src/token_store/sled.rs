use super::Error;
use async_trait::async_trait;
use bytes::{Buf, BufMut, BytesMut};
use chrono::{DateTime, Utc};
use houseflow_types::token::RefreshTokenID;

#[derive(Clone)]
pub struct TokenStore {
    database: sled::Db,
}

impl TokenStore {
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let config = sled::Config::new().path(path);
        Ok(Self {
            database: config.open()?,
        })
    }

    pub fn new_temporary(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let config = sled::Config::new().path(path).temporary(true);
        Ok(Self {
            database: config.open()?,
        })
    }
}

#[async_trait]
impl crate::TokenStore for TokenStore {
    async fn exists(&self, id: &RefreshTokenID) -> Result<bool, Error> {
        match self.database.get(id)? {
            Some(content) => {
                let mut buf = BytesMut::from(content.as_ref());
                let expirable = match buf.get_u8() {
                    0 => false,
                    1 => true,
                    other => {
                        return Err(Error::InvalidData(format!(
                            "{} is not valid `expirable` boolean",
                            other
                        )))
                    }
                };
                if expirable {
                    let expires_at = buf.get_i64();
                    let now = Utc::now().timestamp();
                    if expires_at < now {
                        self.remove(id).await?;
                        Ok(false)
                    } else {
                        Ok(true)
                    }
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }

    async fn remove(&self, id: &RefreshTokenID) -> Result<bool, Error> {
        let removed = self.database.remove(id)?;
        self.database.flush_async().await?;
        Ok(removed.is_some())
    }

    async fn add(
        &self,
        id: &RefreshTokenID,
        expires_at: Option<&DateTime<Utc>>,
    ) -> Result<(), Error> {
        let mut buf = BytesMut::new();
        match expires_at {
            Some(expires_at) => {
                let expirable = true;
                buf.put_u8(expirable.into());
                buf.put_i64(expires_at.timestamp());
            }
            None => {
                let expirable = false;
                buf.put_u8(expirable.into());
            }
        };
        self.database.insert(id, buf.as_ref())?;
        self.database.flush_async().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use super::*;
    use crate::TokenStore;
    use chrono::Duration;

    fn get_token_store() -> super::TokenStore {
        let path = std::env::temp_dir().join(format!(
            "houseflow-token-store_test-{}",
            rand::random::<u32>()
        ));
        super::TokenStore::new_temporary(path).unwrap()
    }

    #[tokio::test]
    async fn add_exists_remove() {
        let token_store = get_token_store();
        let token_id = random();
        token_store
            .add(&token_id, Some(&(Utc::now() + Duration::minutes(10))))
            .await
            .unwrap();
        assert_eq!(token_store.exists(&token_id).await.unwrap(), true);
        token_store.remove(&token_id).await.unwrap();
        assert_eq!(token_store.exists(&token_id).await.unwrap(), false);
    }

    #[tokio::test]
    async fn add_expired() {
        let token_store = get_token_store();
        let token_id = random();
        token_store
            .add(&token_id, Some(&(Utc::now() - Duration::minutes(10))))
            .await
            .unwrap();
        assert_eq!(token_store.exists(&token_id).await.unwrap(), false);
    }
}
