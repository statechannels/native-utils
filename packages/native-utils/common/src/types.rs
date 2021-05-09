use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

use ethereum_types::U256;
use serde::de::{Error as SerdeError, *};
use serde::ser::*;
use serde_derive::*;

use super::tokenize::*;

pub trait ToHexString {
    fn to_hex_string(&self) -> String;
}

impl ToHexString for Vec<u8> {
    fn to_hex_string(&self) -> String {
        format!("0x{}", hex::encode(self))
    }
}

#[derive(PartialEq)]
pub struct Bytes(pub Vec<u8>);

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_hex_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let prefixed: String = Deserialize::deserialize(deserializer)?;
        let unprefixed = prefixed.trim_start_matches("0x");
        hex::decode(unprefixed).map(Bytes).map_err(D::Error::custom)
    }
}

impl Tokenize for Bytes {
    fn tokenize(&self) -> Token {
        Token::Bytes(self.0.clone())
    }
}

#[derive(PartialEq)]
pub struct Bytes32(Vec<u8>);

impl From<[u8; 32]> for Bytes32 {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes.into())
    }
}

impl Deref for Bytes32 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Bytes32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_hex_string().serialize(serializer)
    }
}

impl TryFrom<Bytes> for Bytes32 {
    type Error = String;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        match bytes.len() {
            32 => Ok(Self(bytes.0)),
            n => Err(format!("bytes32 value has incorrect length: {}", n)),
        }
    }
}

impl<'de> Deserialize<'de> for Bytes32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Bytes = Deserialize::deserialize(deserializer)?;
        bytes.try_into().map_err(D::Error::custom)
    }
}

impl Tokenize for Bytes32 {
    fn tokenize(&self) -> Token {
        Token::FixedBytes(self.0.clone())
    }
}

#[derive(PartialEq, PartialOrd)]
pub struct Uint48(pub u64);

impl<'de> Deserialize<'de> for Uint48 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Self)
    }
}

impl Serialize for Uint48 {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
    S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl Tokenize for Uint48 {
    fn tokenize(&self) -> Token {
        Token::Uint(self.0.into())
    }
}

#[derive(PartialEq, Serialize)]
pub struct Uint256(pub U256);

impl From<U256> for Uint256 {
    fn from(n: U256) -> Self {
        Self(n)
    }
}

struct Uint256Visitor;

impl<'de> Visitor<'de> for Uint256Visitor {
    type Value = Uint256;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a decimal uint256 string or unsigned integer")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        if value >= 0 {
            Ok(U256::from(value).into())
        } else {
            Err(SerdeError::custom("negative integer"))
        }
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(U256::from(value).into())
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        if value >= 0 {
            Ok(U256::from(value).into())
        } else {
            Err(SerdeError::custom("negative integer"))
        }
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(U256::from(value).into())
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        if value >= 0 {
            Ok(U256::from(value).into())
        } else {
            Err(SerdeError::custom("negative integer"))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        Ok(U256::from(value).into())
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: SerdeError,
    {
        U256::from_dec_str(s)
            .or_else(|_| U256::from_str(s.trim_start_matches("0x")).map_err(SerdeError::custom))
            .map(Uint256::from)
    }
}

impl<'de> Deserialize<'de> for Uint256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(Uint256Visitor)
    }
}

impl Tokenize for Uint256 {
    fn tokenize(&self) -> Token {
        Token::Uint(self.0)
    }
}
