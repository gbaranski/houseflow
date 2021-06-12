use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use token::Token;

#[derive(Debug, Clone)]
pub struct Keystore {
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeystoreFile {
    pub refresh_token: Token,
    pub access_token: Token,
}

impl Keystore {
    pub async fn remove(&self) -> anyhow::Result<()> {
        if self.path.exists() {
            Ok(tokio::fs::remove_file(&self.path)
                .await
                .with_context(|| "remove keystore file failed")?)
        } else {
            Ok(())
        }
    }

    pub async fn save(&self, keystore_file: &KeystoreFile) -> anyhow::Result<()> {
        use tokio::io::AsyncWriteExt;

        let file_contents = serde_json::to_string_pretty(keystore_file)?;

        if let Some(path) = self.path.parent() {
            if !path.exists() {
                tokio::fs::create_dir_all(path)
                    .await
                    .with_context(|| "create file parents failed")?;
            }
        }

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.path)
            .await
            .with_context(|| "open file failed")?;

        file.set_len(0_u64)
            .await
            .with_context(|| "set length failed")?;
        file.write_all(file_contents.as_bytes())
            .await
            .with_context(|| "write tokens to file failed")?;

        Ok(())
    }

    pub async fn read(&self) -> anyhow::Result<KeystoreFile> {
        if !self.path.exists() {
            return Err(anyhow::Error::msg(format!(
                "keystore file not found at {}",
                self.path.to_str().unwrap_or("INVALID_PATH")
            )));
        }

        let file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(&self.path)
            .await
            .with_context(|| "open file")?;

        let file: KeystoreFile = serde_json::from_reader(file.into_std().await).with_context(|| "deserializing keystore file")?;

        Ok(file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_keystore() {
        let user_id = rand::random();
        let refresh_token =
            Token::new_refresh_token(b"refresh-key", &user_id, &token::UserAgent::Internal);
        let access_token =
            Token::new_refresh_token(b"access-key", &user_id, &token::UserAgent::Internal);
        let keystore_file = KeystoreFile {
            refresh_token,
            access_token,
        };

        let path_string = format!(
            "/tmp/houseflow/tokens-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        let path = std::path::Path::new(&path_string);

        let keystore = Keystore { path: path.into() };

        keystore.save(&keystore_file).await.unwrap();
        keystore.save(&keystore_file).await.unwrap();
        let read_keystore_file = keystore.read().await.unwrap();
        assert_eq!(keystore_file, read_keystore_file);
        keystore.remove().await.unwrap();
        assert!(path.exists() == false);
        keystore.remove().await.unwrap();
        assert!(keystore.read().await.is_err());
    }
}
