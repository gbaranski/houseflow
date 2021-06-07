use houseflow_auth_types::{
    AccessTokenError, AccessTokenRequest, AccessTokenResponse, GrantType, LoginError, LoginRequest,
    LoginResponse, RegisterError, RegisterRequest, RegisterResponse, WhoamiResponse
};
use houseflow_token::Token;
use reqwest::Client;
use thiserror::Error;
use url::Url;

#[cfg(any(feature = "keystore", test))]
#[derive(Debug, Clone)]
pub struct KeystoreConfig {
    pub path: std::path::PathBuf,
}

#[cfg(any(feature = "keystore", test))]
#[derive(Debug, thiserror::Error)]
pub enum KeystoreError {
    #[error("open failed: `{0}`")]
    OpenError(tokio::io::Error),

    #[error("read failed: `{0}`")]
    ReadError(tokio::io::Error),

    #[error("write failed: `{0}`")]
    WriteError(tokio::io::Error),

    #[error("create parents failed: `{0}`")]
    CreateParentsError(tokio::io::Error),

    #[error("remove failed: `{0}`")]
    RemoveError(tokio::io::Error),

    #[error("invalid token: `{0}`")]
    InvalidToken(houseflow_token::DecodeError),
}

#[derive(Clone)]
pub struct Auth {
    pub url: Url,

    #[cfg(any(feature = "keystore", test))]
    pub keystore: KeystoreConfig,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("error occured with sending request: `{0}`")]
    ReqwestError(#[from] reqwest::Error),

    #[error("refreshing access token failed with: `{0}`")]
    RefreshAccessTokenError(#[from] AccessTokenError),

    #[error("not logged in")]
    NotLoggedIn,

    #[error("registration failed: `{0}`")]
    RegisterError(#[from] RegisterError),

    #[error("login failed: `{0}`")]
    LoginError(#[from] LoginError),

    #[cfg(any(feature = "keystore", test))]
    #[error("keystore error: `{0}`")]
    KeystoreError(#[from] KeystoreError),
}

impl Auth {
    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, Error> {
        let client = Client::new();
        let url = self.url.join("register").unwrap();

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<RegisterResponse>()
            .await?;

        Ok(response)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, Error> {
        let client = Client::new();
        let url = self.url.join("login").unwrap();

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<LoginResponse>()
            .await?;

        Ok(response)
    }

    pub async fn fetch_access_token(
        &self,
        refresh_token: &Token,
    ) -> Result<AccessTokenResponse, Error> {
        let client = Client::new();
        let request = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.clone(),
        };
        let url = self.url.join("token").unwrap();

        let response = client
            .post(url)
            .query(&request)
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;

        Ok(response)
    }

    pub async fn whoami(&self, access_token: &Token) -> Result<WhoamiResponse, Error> {
        let client = Client::new();
        let url = self.url.join("whoami").unwrap();

        let response = client
            .get(url)
            .bearer_auth(access_token.to_string())
            .send()
            .await?
            .json::<WhoamiResponse>()
            .await?;

        Ok(response)
    }

    #[cfg(any(feature = "keystore", test))]
    pub async fn remove_refresh_token(&self) -> Result<(), Error> {
        if self.keystore.path.exists() {
            Ok(tokio::fs::remove_file(&self.keystore.path)
                .await
                .map_err(|err| KeystoreError::RemoveError(err))?)
        } else {
            Ok(())
        }
    }

    #[cfg(any(feature = "keystore", test))]
    pub async fn save_refresh_token(&self, refresh_token: &Token) -> Result<(), Error> {
        use tokio::io::AsyncWriteExt;

        if let Some(path) = self.keystore.path.parent() {
            if path.exists() == false {
                tokio::fs::create_dir_all(path)
                    .await
                    .map_err(|err| KeystoreError::CreateParentsError(err))?;
            }
        }

        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.keystore.path)
            .await
            .map_err(|err| KeystoreError::OpenError(err))?;

        file.set_len(0_u64)
            .await
            .map_err(|err| KeystoreError::WriteError(err))?;

        file.write_all(refresh_token.to_string().as_bytes())
            .await
            .map_err(|err| KeystoreError::WriteError(err))?;

        Ok(())
    }

    #[cfg(any(feature = "keystore", test))]
    pub async fn read_refresh_token(&self) -> Result<Option<Token>, Error> {
        use tokio::io::AsyncReadExt;

        if self.keystore.path.exists() == false {
            return Ok(None);
        }

        let mut file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(&self.keystore.path)
            .await
            .map_err(|err| KeystoreError::OpenError(err))?;

        let mut string = String::with_capacity(Token::BASE64_SIZE);
        file.read_to_string(&mut string)
            .await
            .map_err(|err| KeystoreError::ReadError(err))?;
        let token = Token::from_str(&string).map_err(|err| KeystoreError::InvalidToken(err))?;
        Ok(Some(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_keystore() {
        let token = Token::new_refresh_token(
            b"some-key",
            &rand::random(),
            &houseflow_token::UserAgent::Internal,
        );

        let path_string = format!(
            "/tmp/houseflow/tokens-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        let path = std::path::Path::new(&path_string);

        let auth = Auth {
            url: Url::parse("http://localhost:8080").unwrap(),
            keystore: KeystoreConfig { path: path.into() },
        };

        auth.save_refresh_token(&token).await.unwrap();
        auth.save_refresh_token(&token).await.unwrap();
        let read_token = auth.read_refresh_token().await.unwrap().unwrap();
        assert_eq!(token, read_token);
        auth.remove_refresh_token().await.unwrap();
        assert!(path.exists() == false);
        auth.remove_refresh_token().await.unwrap();
        assert_eq!(auth.read_refresh_token().await.unwrap(), None);
    }
}
