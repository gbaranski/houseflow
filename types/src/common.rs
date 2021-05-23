use bytes::{Buf, BufMut};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};
use thiserror::Error;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Credential<const N: usize> {
    inner: [u8; N],
}

impl<const N: usize> Credential<N> {
    pub const SIZE: usize = N;

    pub fn into_bytes(self) -> [u8; N] {
        self.inner
    }

    pub fn from_bytes(bytes: [u8; N]) -> Self {
        Self::from(bytes)
    }

    pub fn to_buf(&self, buf: &mut impl BufMut) {
        buf.put_slice(&self.inner);
    }

    pub fn from_buf(buf: &mut impl Buf) -> Result<Self, CredentialError> {
        if buf.remaining() < N {
            return Err(CredentialError::InvalidSize {
                expected: N,
                received: buf.remaining(),
            });
        }

        let mut inner = [0; N];
        buf.copy_to_slice(&mut inner);
        Ok(Self { inner })
    }
}

impl<const N: usize> AsRef<[u8]> for Credential<N> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

#[derive(Debug, Error)]
pub enum CredentialError {
    #[error("Invalid size, expected: {expected}, received: {received}")]
    InvalidSize { expected: usize, received: usize },

    #[error("Invalid encoding: {0}")]
    InvalidEncoding(#[from] hex::FromHexError),
}

impl<const N: usize> From<[u8; N]> for Credential<N> {
    fn from(v: [u8; N]) -> Self {
        Self { inner: v }
    }
}

impl<const N: usize> Into<[u8; N]> for Credential<N> {
    fn into(self) -> [u8; N] {
        self.inner
    }
}

impl<const N: usize> Default for Credential<N> {
    fn default() -> Self {
        Self { inner: [0; N] }
    }
}

impl<const N: usize> Into<String> for Credential<N> {
    fn into(self) -> String {
        hex::encode(self.inner)
    }
}

impl<const N: usize> TryFrom<&str> for Credential<N> {
    type Error = CredentialError;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        // N * 2 because encoding with hex doubles the size

        if v.len() != N * 2 {
            Err(CredentialError::InvalidSize {
                expected: N * 2,
                received: v.len(),
            })
        } else {
            Ok(Self {
                inner: hex::decode(v)?.try_into().unwrap(),
            })
        }
    }
}

impl<const N: usize> TryFrom<String> for Credential<N> {
    type Error = CredentialError;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        // N * 2 because encoding with hex doubles the size

        if v.len() != N * 2 {
            Err(CredentialError::InvalidSize {
                expected: N * 2,
                received: v.len(),
            })
        } else {
            Ok(Self {
                inner: hex::decode(v)?.try_into().unwrap(),
            })
        }
    }
}

impl<const N: usize> FromStr for Credential<N> {
    type Err = CredentialError;

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        // N * 2 because encoding with hex doubles the size

        if v.len() != N * 2 {
            Err(CredentialError::InvalidSize {
                expected: N * 2,
                received: v.len(),
            })
        } else {
            Ok(Self {
                inner: hex::decode(v)?.try_into().unwrap(),
            })
        }
    }
}

impl<const N: usize> std::fmt::Display for Credential<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", hex::encode(self.inner))
    }
}

impl<const N: usize> std::fmt::Debug for Credential<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Inner: `{}`", hex::encode(self.inner))
    }
}

impl<const N: usize> rand::distributions::Distribution<Credential<N>>
    for rand::distributions::Standard
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Credential<N> {
        Credential {
            inner: (0..N)
                .map(|_| rng.gen())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[cfg(feature = "postgres-types")]
impl<const N: usize> postgres_types::ToSql for Credential<N> {
    fn accepts(ty: &postgres_types::Type) -> bool {
        *ty == postgres_types::Type::BPCHAR
    }

    fn to_sql(
        &self,
        _ty: &postgres_types::Type,
        out: &mut bytes::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        let string = self.to_string();
        out.put_slice(string.as_bytes());
        Ok(postgres_types::IsNull::No)
    }

    fn to_sql_checked(
        &self,
        _ty: &postgres_types::Type,
        out: &mut bytes::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        let string = self.to_string();
        out.put_slice(string.as_bytes());
        Ok(postgres_types::IsNull::No)
    }
}

#[cfg(feature = "postgres-types")]
impl<'a, const N: usize> postgres_types::FromSql<'a> for Credential<N> {
    fn from_sql(
        _ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let str = std::str::from_utf8(raw)?;
        let credential = Self::from_str(str)?;
        Ok(credential)
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        *ty == postgres_types::Type::BPCHAR
    }
}

#[cfg(feature = "serde")]
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde")]
struct CredentialVisitor<const N: usize>;

#[cfg(feature = "serde")]
impl<'de, const N: usize> Visitor<'de> for CredentialVisitor<N> {
    type Value = [u8; N];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("an array of length {}", N))
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for Credential<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_bytes(CredentialVisitor::<N>)
            .map(|bytes| bytes.try_into().unwrap())
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for Credential<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;
    const SIZE: usize = 32;

    #[test]
    fn test_buffer_parse() {
        let mut buf = BytesMut::with_capacity(SIZE);
        let credential: Credential<SIZE> = rand::random();
        credential.to_buf(&mut buf);
        let parsed_credential = Credential::<SIZE>::from_buf(&mut buf)
            .expect("reading Credential from buffer returned Error");
        assert_eq!(credential, parsed_credential);
    }

    #[test]
    fn test_buffer_parse_underflow() {
        let mut buf = BytesMut::with_capacity(SIZE);
        let credential: Credential<SIZE> = rand::random();
        credential.to_buf(&mut buf);
        buf = buf[0..SIZE - 1].into(); // Malform some last bytes of Buf
        Credential::<SIZE>::from_buf(&mut buf)
            .expect_err("reading malformed Credential from buffer did not return Error");
    }
}
