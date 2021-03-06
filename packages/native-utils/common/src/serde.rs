use hex;
use serde::ser::{Serialize, Serializer};
use serde::de::{Error, Deserialize, Deserializer};

use super::state::RecoverableSignature;
use super::types::*;

impl Serialize for RecoverableSignature {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
    S: Serializer,
    {
        // A helper struct to go from `[u8; 64]` to `&[u8]` so that `hex::encode`
        // accepts it.
        struct RawBytes64([u8; 64]);

        impl AsRef<[u8]> for RawBytes64 {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        let bytes = self.0.serialize();
        let s = hex::encode(RawBytes64(bytes));
        serializer.serialize_str(format!("0x{}{:x}", s, self.1.serialize() + 27).as_str())
    }
}

impl<'de> Deserialize<'de> for RecoverableSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Bytes = Deserialize::deserialize(deserializer)?;
        Ok(RecoverableSignature::from_bytes(bytes).map_err(D::Error::custom)?)
    }
}
