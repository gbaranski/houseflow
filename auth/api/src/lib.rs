use houseflow_auth_types::{
    AccessTokenError, AccessTokenRequest, AccessTokenResponse, GrantType, LoginError, LoginRequest,
    LoginResponse, LoginResponseBody, RegisterError, RegisterRequest, RegisterResponse,
    RegisterResponseBody,
};
use houseflow_token::Token;
use reqwest::Client;
use thiserror::Error;
use url::Url;

#[cfg(any(feature = "token_store", test))]
#[derive(Clone)]
pub struct TokenStoreConfig {
    pub path: std::path::PathBuf,
}

#[cfg(any(feature = "token_store", test))]
#[derive(Debug, thiserror::Error)]
pub enum TokenStoreError {
    #[error("store open failed: `{0}`")]
    OpenError(tokio::io::Error),

    #[error("store read failed: `{0}`")]
    ReadError(tokio::io::Error),

    #[error("store write failed: `{0}`")]
    WriteError(tokio::io::Error),

    #[error("store remove failed: `{0}`")]
    RemoveError(tokio::io::Error),

    #[error("invalid token: `{0}`")]
    InvalidToken(houseflow_token::DecodeError),
}

#[derive(Clone)]
pub struct AuthConfig {
    pub url: Url,

    #[cfg(any(feature = "token_store", test))]
    pub token_store: TokenStoreConfig,
}

#[derive(Clone)]
pub struct Auth {
    config: AuthConfig,
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

    #[cfg(any(feature = "token_store", test))]
    #[error("token store error: `{0}`")]
    TokenStoreError(#[from] TokenStoreError),
}

impl Auth {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, Error> {
        let client = Client::new();
        let url = self.config.url.join("register").unwrap();

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<RegisterResponse>()
            .await?;

        Ok(response)
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponseBody, Error> {
        let client = Client::new();
        let url = self.config.url.join("login").unwrap();

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json::<LoginResponse>()
            .await??;

        Ok(response)
    }

    pub async fn fetch_access_token(url: &Url, refresh_token: &Token) -> Result<Token, Error> {
        let client = Client::new();
        let request = AccessTokenRequest {
            grant_type: GrantType::RefreshToken,
            refresh_token: refresh_token.clone(),
        };
        let url = url.join("token").unwrap();

        let response = client
            .post(url)
            .query(&request)
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await??;

        Ok(response.access_token)
    }

    #[cfg(any(feature = "token_store", test))]
    pub async fn remove_refresh_token(&self) -> Result<(), Error> {
        if self.config.token_store.path.exists() {
            Ok(tokio::fs::remove_file(&self.config.token_store.path)
                .await
                .map_err(|err| TokenStoreError::RemoveError(err))?)
        } else {
            Ok(())
        }
    }

    #[cfg(any(feature = "token_store", test))]
    pub async fn save_refresh_token(&self, refresh_token: &Token) -> Result<(), Error> {
        use tokio::io::AsyncWriteExt;

        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.config.token_store.path)
            .await
            .map_err(|err| TokenStoreError::OpenError(err))?;

        file.set_len(0_u64)
            .await
            .map_err(|err| TokenStoreError::WriteError(err))?;

        file.write_all(refresh_token.to_string().as_bytes())
            .await
            .map_err(|err| TokenStoreError::WriteError(err))?;

        Ok(())
    }

    #[cfg(any(feature = "token_store", test))]
    pub async fn read_refresh_token(&self) -> Result<Option<Token>, Error> {
        use tokio::io::AsyncReadExt;

        if self.config.token_store.path.exists() == false {
            return Ok(None);
        }

        let mut file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(&self.config.token_store.path)
            .await
            .map_err(|err| TokenStoreError::OpenError(err))?;

        let mut string = String::with_capacity(Token::BASE64_SIZE);
        file.read_to_string(&mut string)
            .await
            .map_err(|err| TokenStoreError::ReadError(err))?;
        let token = Token::from_str(&string).map_err(|err| TokenStoreError::InvalidToken(err))?;
        Ok(Some(token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_token_store() {
        let token = Token::new_refresh_token(
            b"some-key",
            &rand::random(),
            &houseflow_token::UserAgent::Internal,
        );

        let path_string = format!(
            "/tmp/houseflow-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        let path = std::path::Path::new(&path_string);

        let token_store_config = TokenStoreConfig { path: path.into() };
        let auth_config = AuthConfig {
            url: Url::parse("http://localhost:80").unwrap(),
            token_store: token_store_config,
        };
        let auth = Auth::new(auth_config);

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
