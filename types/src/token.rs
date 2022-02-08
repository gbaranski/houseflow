use crate::errors::TokenError as Error;
use chrono::DateTime;
use chrono::Utc;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::de;
use serde::ser;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct Token<C: ser::Serialize + de::DeserializeOwned> {
    header: Header,
    pub claims: C,
    encoded: String,
}

impl<C: ser::Serialize + de::DeserializeOwned> From<Token<C>> for TokenData<C> {
    fn from(token: Token<C>) -> Self {
        Self {
            header: token.header,
            claims: token.claims,
        }
    }
}

impl<C: ser::Serialize + de::DeserializeOwned> std::fmt::Display for Token<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.encoded)
    }
}

impl<C: ser::Serialize + de::DeserializeOwned> std::fmt::Debug for Token<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.encoded)
    }
}

impl<C: ser::Serialize + de::DeserializeOwned> std::ops::Deref for Token<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.claims
    }
}

pub type AccessToken = Token<AccessTokenClaims>;
pub type RefreshToken = Token<RefreshTokenClaims>;
pub type AuthorizationCode = Token<AuthorizationCodeClaims>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationCodeClaims {
    pub sub: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub exp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseClaims {
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub exp: Option<DateTime<Utc>>,
}

impl<C: ser::Serialize + de::DeserializeOwned> Token<C> {
    pub fn new(key: &[u8], claims: C) -> Result<Self, Error> {
        let header = Header::new(Algorithm::HS256);
        let encoded = encode(&header, &claims, &EncodingKey::from_secret(key))?;

        Ok(Self {
            header,
            encoded,
            claims,
        })
    }

    pub fn encode(&self) -> String {
        self.encoded.clone()
    }

    /// Validate the expiry (if it is present) but not the signature.
    pub fn decode_insecure(token: &str) -> Result<Self, Error> {
        let mut validation = Validation::default();
        validation.insecure_disable_signature_validation();
        let data: TokenData<C> = decode(token, &DecodingKey::from_secret(&[]), &validation)?;

        Ok(Self {
            header: data.header,
            claims: data.claims,
            encoded: token.to_owned(),
        })
    }

    /// Don't validate anything.
    pub fn decode_insecure_novalidate(token: &str) -> Result<Self, Error> {
        let mut validation = Validation::default();
        validation.validate_exp = false;
        validation.insecure_disable_signature_validation();
        let data = decode(token, &DecodingKey::from_secret(&[]), &validation)?;
        Ok(Self {
            header: data.header,
            claims: data.claims,
            encoded: token.to_owned(),
        })
    }

    /// Validate the signature, and the expiry if it is present.
    pub fn decode(key: &[u8], token: &str) -> Result<Self, Error> {
        let validation = Validation::default();
        let data: TokenData<C> = decode(token, &DecodingKey::from_secret(key), &validation)?;

        Ok(Self {
            header: data.header,
            claims: data.claims,
            encoded: token.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::SubsecRound;
    fn get_key() -> Vec<u8> {
        use rand::RngCore;
        let mut bytes = [0; 32];
        rand::thread_rng().fill_bytes(&mut bytes);

        Vec::from(bytes)
    }

    mod at {
        use super::*;

        #[test]
        fn valid() {
            let key = get_key();
            let payload = AccessTokenClaims {
                sub: Uuid::new_v4(),
                exp: Utc::now().round_subsecs(0) + chrono::Duration::hours(1),
            };
            let token = AccessToken::new(&key, payload).unwrap();
            let encoded = token.encode();
            let decoded = AccessToken::decode(&key, &encoded).unwrap();
            assert_eq!(token.header, decoded.header);
            assert_eq!(token.claims, decoded.claims);
        }

        #[test]
        fn expired() {
            let key = get_key();
            let expired_by = chrono::Duration::hours(1);
            let payload = AccessTokenClaims {
                sub: Uuid::new_v4(),
                exp: Utc::now() - expired_by,
            };
            let token = AccessToken::new(&key, payload).unwrap();
            let encoded = token.encode();
            let err = Token::<AccessTokenClaims>::decode(&key, &encoded).unwrap_err();
            assert_eq!(
                err,
                Error {
                    description: "ExpiredSignature".to_string(),
                }
            );
        }

        #[test]
        fn invalid_signature() {
            let valid_key = get_key();
            let invalid_key = get_key();
            let payload = AccessTokenClaims {
                sub: Uuid::new_v4(),
                exp: Utc::now() - chrono::Duration::hours(1),
            };
            let token = AccessToken::new(&valid_key, payload).unwrap();
            let encoded = token.encode();
            let err = AccessToken::decode(&invalid_key, &encoded).unwrap_err();
            assert_eq!(
                err,
                Error {
                    description: "InvalidSignature".to_string()
                }
            );
        }
    }

    mod rt {
        use super::*;

        #[test]
        fn valid_with_exp() {
            let key = get_key();
            let payload = RefreshTokenClaims {
                sub: Uuid::new_v4(),
                exp: Some(Utc::now().round_subsecs(0) + chrono::Duration::hours(1)),
            };
            let token = RefreshToken::new(&key, payload).unwrap();
            let encoded = token.encode();
            let decoded = RefreshToken::decode(&key, &encoded).unwrap();
            assert_eq!(token.header, decoded.header);
            assert_eq!(token.claims, decoded.claims);
        }

        #[test]
        fn valid_without_exp() {
            let key = get_key();
            let payload = RefreshTokenClaims {
                sub: Uuid::new_v4(),
                exp: None,
            };
            let token = RefreshToken::new(&key, payload).unwrap();
            let encoded = token.encode();
            let decoded = RefreshToken::decode(&key, &encoded).unwrap();
            assert_eq!(token.header, decoded.header);
            assert_eq!(token.claims, decoded.claims);
        }

        #[test]
        fn expired() {
            let key = get_key();
            let expired_by = chrono::Duration::hours(1);
            let payload = RefreshTokenClaims {
                sub: Uuid::new_v4(),
                exp: Some(Utc::now() - expired_by),
            };
            let token = Token::new(&key, payload).unwrap();
            let encoded = token.encode();
            let err = RefreshToken::decode(&key, &encoded).unwrap_err();
            assert_eq!(
                err,
                Error {
                    description: "ExpiredSignature".to_string()
                }
            );
        }

        #[test]
        fn invalid_signature() {
            let valid_key = get_key();
            let invalid_key = get_key();
            let payload = RefreshTokenClaims {
                sub: Uuid::new_v4(),
                exp: Some(Utc::now().round_subsecs(0) + chrono::Duration::hours(1)),
            };
            let token = RefreshToken::new(&valid_key, payload).unwrap();
            let encoded = token.encode();
            let err = RefreshToken::decode(&invalid_key, &encoded).unwrap_err();
            assert_eq!(
                err,
                Error {
                    description: "InvalidSignature".to_string()
                }
            );
        }
    }
}
