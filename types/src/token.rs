use crate::errors::TokenError as Error;
use chrono::DateTime;
use chrono::Utc;
use jsonwebtoken::dangerous_insecure_decode_with_validation;
use jsonwebtoken::{
    dangerous_insecure_decode, decode, encode, Algorithm, DecodingKey, EncodingKey, Header,
    TokenData, Validation,
};
use serde::de;
use serde::ser;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub struct Token<P: ser::Serialize + de::DeserializeOwned> {
    header: Header,
    payload: P,
    encoded: String,
}

impl<P: ser::Serialize + de::DeserializeOwned> From<Token<P>> for TokenData<P> {
    fn from(token: Token<P>) -> Self {
        Self {
            header: token.header,
            claims: token.payload,
        }
    }
}

impl<P: ser::Serialize + de::DeserializeOwned> std::fmt::Display for Token<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.encoded)
    }
}

impl<P: ser::Serialize + de::DeserializeOwned> std::fmt::Debug for Token<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.encoded)
    }
}

impl<P: ser::Serialize + de::DeserializeOwned> std::ops::Deref for Token<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.payload
    }
}

pub type AccessToken = Token<AccessTokenPayload>;
pub type RefreshToken = Token<RefreshTokenPayload>;
pub type AuthorizationCode = Token<AuthorizationCodePayload>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessTokenPayload {
    pub sub: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationCodePayload {
    pub sub: Uuid,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefreshTokenPayload {
    pub sub: Uuid,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub exp: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct BasePayload {
    #[serde(with = "chrono::serde::ts_seconds_option")]
    exp: Option<DateTime<Utc>>,
}

impl<P: ser::Serialize + de::DeserializeOwned> Token<P> {
    pub fn new(key: &[u8], payload: P) -> Result<Self, Error> {
        const ALGORITHM: Algorithm = Algorithm::HS256; // that can be changed in the future

        let header = Header::new(ALGORITHM);
        let encoded = encode(&header, &payload, &EncodingKey::from_secret(key))?;

        Ok(Self {
            header,
            payload,
            encoded,
        })
    }

    pub fn encode(&self) -> String {
        self.encoded.clone()
    }

    /// Validate the expiry (if it is present) but not the signature.
    pub fn decode_unsafe(token: &str) -> Result<Self, Error> {
        // Hack to allow tokens without "exp", but validate it if it is present.
        let unvalidated_data: TokenData<BasePayload> = dangerous_insecure_decode(token)?;
        let validation = Validation {
            validate_exp: unvalidated_data.claims.exp.is_some(),
            ..Validation::default()
        };

        let data = dangerous_insecure_decode_with_validation(token, &validation)?;
        Ok(Self {
            header: data.header,
            payload: data.claims,
            encoded: token.to_owned(),
        })
    }

    /// Don't validate anything.
    pub fn decode_unsafe_novalidate(token: &str) -> Result<Self, Error> {
        let data = dangerous_insecure_decode(token)?;
        Ok(Self {
            header: data.header,
            payload: data.claims,
            encoded: token.to_owned(),
        })
    }

    /// Validate the signature, and the expiry if it is present.
    pub fn decode(key: &[u8], token: &str) -> Result<TokenData<P>, Error> {
        // Hack to allow tokens without "exp", but validate it if it is present.
        let unvalidated_data: TokenData<BasePayload> = dangerous_insecure_decode(token)?;
        let validation = Validation {
            validate_exp: unvalidated_data.claims.exp.is_some(),
            ..Validation::default()
        };

        Ok(decode(token, &DecodingKey::from_secret(key), &validation)?)
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
            let payload = AccessTokenPayload {
                sub: Uuid::new_v4(),
                exp: Utc::now().round_subsecs(0) + chrono::Duration::hours(1),
            };
            let token = AccessToken::new(&key, payload).unwrap();
            let encoded = token.encode();
            let decoded = AccessToken::decode(&key, &encoded).unwrap();
            assert_eq!(token.header, decoded.header);
            assert_eq!(token.payload, decoded.claims);
        }

        #[test]
        fn expired() {
            let key = get_key();
            let expired_by = chrono::Duration::hours(1);
            let payload = AccessTokenPayload {
                sub: Uuid::new_v4(),
                exp: Utc::now() - expired_by,
            };
            let token = AccessToken::new(&key, payload).unwrap();
            let encoded = token.encode();
            let err = Token::<AccessTokenPayload>::decode(&key, &encoded).unwrap_err();
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
            let payload = AccessTokenPayload {
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
            let payload = RefreshTokenPayload {
                sub: Uuid::new_v4(),
                exp: Some(Utc::now().round_subsecs(0) + chrono::Duration::hours(1)),
            };
            let token = RefreshToken::new(&key, payload).unwrap();
            let encoded = token.encode();
            let decoded = RefreshToken::decode(&key, &encoded).unwrap();
            assert_eq!(token.header, decoded.header);
            assert_eq!(token.payload, decoded.claims);
        }

        #[test]
        fn valid_without_exp() {
            let key = get_key();
            let payload = RefreshTokenPayload {
                sub: Uuid::new_v4(),
                exp: None,
            };
            let token = RefreshToken::new(&key, payload).unwrap();
            let encoded = token.encode();
            let decoded = RefreshToken::decode(&key, &encoded).unwrap();
            assert_eq!(token.header, decoded.header);
            assert_eq!(token.payload, decoded.claims);
        }

        #[test]
        fn expired() {
            let key = get_key();
            let expired_by = chrono::Duration::hours(1);
            let payload = RefreshTokenPayload {
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
            let payload = RefreshTokenPayload {
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
