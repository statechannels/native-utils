use ethereum_types::{U256, U64};
use hex;
use secp256k1::{RecoveryId, Signature};
use serde::de::*;
use serde::ser::*;

pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<U64, D::Error>
where
    D: Deserializer<'de>,
{
    let n: u64 = Deserialize::deserialize(deserializer)?;
    Ok(U64::from(n))
}

pub fn deserialize_u256_number<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let n: u64 = Deserialize::deserialize(deserializer)?;
    Ok(U256::from(n))
}

pub fn deserialize_u256_string<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    U256::from_dec_str(s.as_str()).map_err(D::Error::custom)
}

pub fn serialize_bytes_string<S>(bytes: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    format!("0x{}", hex::encode(bytes)).serialize(serializer)
}

pub fn deserialize_bytes_string<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let prefixed: String = Deserialize::deserialize(deserializer)?;
    let unprefixed = prefixed.trim_start_matches("0x");
    hex::decode(unprefixed).map_err(D::Error::custom)
}

pub fn deserialize_bytes_strings<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<String> = Deserialize::deserialize(deserializer)?;
    strings.into_iter().try_fold(vec![], |mut acc, s| {
        let unprefixed = s.trim_start_matches("0x");
        let bytes = hex::decode(unprefixed).map_err(D::Error::custom)?;
        acc.push(bytes);
        Ok(acc)
    })
}

pub fn serialize_signature<S>(
    (signature, recovery_id): &(Signature, RecoveryId),
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // A helper struct to go from `[u8; 64]` to `&[u8]` so that /`hex::encode`
    // accepts it.
    struct RawBytes64([u8; 64]);

    impl AsRef<[u8]> for RawBytes64 {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    let bytes = signature.serialize();
    let s = hex::encode(RawBytes64(bytes));
    serializer.serialize_str(format!("0x{}{:x}", s, recovery_id.serialize() + 27).as_str())
}
