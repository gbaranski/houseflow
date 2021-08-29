use super::Error;
use async_trait::async_trait;
use bytes::{Buf, BufMut, BytesMut};
use chrono::{DateTime, Utc};
use houseflow_types::token::RefreshTokenID;
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref CLEAN_EXPIRED_INTERVAL: std::time::Duration =
        chrono::Duration::hours(12).to_std().unwrap();
}

#[derive(Clone)]
pub struct TokenBlacklist {
    database: sled::Db,
    clean_expired_handle: Arc<Mutex<Option<tokio::task::JoinHandle<Result<(), Error>>>>>,
}

impl TokenBlacklist {
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let config = sled::Config::new().path(path);
        Self::with_config(config)
    }

    pub fn new_temporary(path: impl AsRef<std::path::Path>) -> Result<Self, Error> {
        let config = sled::Config::new().path(path).temporary(true);
        Self::with_config(config)
    }

    pub fn with_config(config: sled::Config) -> Result<Self, Error> {
        let this = Self {
            database: config.open()?,
            clean_expired_handle: Arc::new(Mutex::new(None)),
        };
        let clean_expired_handle = {
            let this = this.clone();
            tokio::spawn(async move { this.clean_expired_loop().await })
        };
        *this.clean_expired_handle.lock().unwrap() = Some(clean_expired_handle);
        Ok(this)
    }

    async fn clean_expired_loop(&self) -> Result<(), Error> {
        loop {
            crate::TokenBlacklist::remove_expired(self).await?;
            tokio::time::sleep(*CLEAN_EXPIRED_INTERVAL).await;
        }
    }
}

#[derive(Debug)]
struct BlacklistEntryValue {
    expires_at: Option<DateTime<Utc>>,
}

impl BlacklistEntryValue {
    pub fn has_expired(&self) -> bool {
        match self.expires_at {
            Some(expires_at) => expires_at.timestamp() < Utc::now().timestamp(),
            None => false,
        }
    }

    pub fn to_bytes(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        match self.expires_at {
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
        buf
    }
}

impl TryFrom<&sled::IVec> for BlacklistEntryValue {
    type Error = Error;

    fn try_from(value: &sled::IVec) -> Result<Self, Self::Error> {
        let mut buf = BytesMut::from(value.as_ref());
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
            let secs = buf.get_i64();
            use chrono::prelude::NaiveDateTime;
            Ok(Self {
                expires_at: Some(DateTime::from_utc(
                    NaiveDateTime::from_timestamp(secs, 0),
                    Utc,
                )),
            })
        } else {
            Ok(Self { expires_at: None })
        }
    }
}

#[async_trait]
impl crate::TokenBlacklist for TokenBlacklist {
    async fn exists(&self, id: &RefreshTokenID) -> Result<bool, Error> {
        match self.database.get(id)? {
            Some(content) => {
                let value = BlacklistEntryValue::try_from(&content)?;
                if value.has_expired() {
                    Ok(false)
                } else {
                    Ok(true)
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

    async fn remove_expired(&self) -> Result<(), Error> {
        // TODO: Handle errors
        self.database
            .iter()
            .filter_map(|token| {
                let (key, value) = token.as_ref().unwrap();
                let value = BlacklistEntryValue::try_from(value).unwrap();
                if value.has_expired() {
                    Some(key.clone())
                } else {
                    None
                }
            })
            .for_each(|token_id| {
                self.database.remove(&token_id).unwrap();
            });
        Ok(())
    }

    async fn add(
        &self,
        id: &RefreshTokenID,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<(), Error> {
        let value = BlacklistEntryValue { expires_at };
        self.database.insert(id, value.to_bytes().as_ref())?;
        self.database.flush_async().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rand::random;

    use super::*;
    use crate::TokenBlacklist as _;
    use chrono::Duration;

    fn get_token_blacklist() -> TokenBlacklist {
        let path = std::env::temp_dir().join(format!(
            "houseflow-token-blacklist-{}",
            rand::random::<u32>()
        ));
        TokenBlacklist::new_temporary(path).unwrap()
    }

    #[tokio::test]
    async fn add_exists_remove() {
        let token_blacklist = get_token_blacklist();
        let token_id = random();
        token_blacklist
            .add(&token_id, Some(Utc::now() + Duration::minutes(10)))
            .await
            .unwrap();
        assert_eq!(token_blacklist.exists(&token_id).await.unwrap(), true);
        token_blacklist.remove(&token_id).await.unwrap();
        assert_eq!(token_blacklist.exists(&token_id).await.unwrap(), false);
    }

    #[tokio::test]
    async fn add_exists_remove_unexpirable() {
        let token_blacklist = get_token_blacklist();
        let token_id = random();
        token_blacklist.add(&token_id, None).await.unwrap();
        assert_eq!(token_blacklist.exists(&token_id).await.unwrap(), true);
        token_blacklist.remove(&token_id).await.unwrap();
        assert_eq!(token_blacklist.exists(&token_id).await.unwrap(), false);
    }

    #[tokio::test]
    async fn add_expired() {
        let token_blacklist = get_token_blacklist();
        let token_id = random();
        token_blacklist
            .add(&token_id, Some(Utc::now() - Duration::minutes(10)))
            .await
            .unwrap();
        assert_eq!(token_blacklist.exists(&token_id).await.unwrap(), false);
    }

    #[tokio::test]
    async fn remove_expired() {
        let token_blacklist = get_token_blacklist();
        let token_id_expired = random();
        token_blacklist
            .add(&token_id_expired, Some(Utc::now() - Duration::minutes(10)))
            .await
            .unwrap();

        let token_id = random();
        token_blacklist
            .add(&token_id, Some(Utc::now() + Duration::minutes(10)))
            .await
            .unwrap();

        token_blacklist.remove_expired().await.unwrap();
        assert_eq!(
            token_blacklist.exists(&token_id_expired).await.unwrap(),
            false
        );
        assert_eq!(token_blacklist.exists(&token_id).await.unwrap(), true);
    }
}
