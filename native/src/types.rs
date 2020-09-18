use std::convert::TryInto;
use std::ops::Deref;

use ethabi::{encode, Token};
use ethereum_types::{Address, U256, U64};
use hex;
use secp256k1::{sign, Message, RecoveryId, SecretKey, Signature};
use serde::de::*;
use serde::ser::*;
use serde_derive::*;

use super::keccak256;

fn deserialize_u64<'de, D>(deserializer: D) -> Result<U64, D::Error>
where
    D: Deserializer<'de>,
{
    let n: u64 = Deserialize::deserialize(deserializer)?;
    Ok(U64::from(n))
}

fn deserialize_u256_number<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let n: u64 = Deserialize::deserialize(deserializer)?;
    Ok(U256::from(n))
}

fn deserialize_u256_string<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    U256::from_dec_str(s.as_str()).map_err(D::Error::custom)
}

fn deserialize_bytes_string<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let prefixed: String = Deserialize::deserialize(deserializer)?;
    let unprefixed = prefixed.trim_start_matches("0x");
    hex::decode(unprefixed).map_err(D::Error::custom)
}

fn deserialize_bytes_strings<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
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

fn serialize_signature<S>(
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

pub struct Bytes(Vec<u8>);

impl Deref for Bytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_bytes_string(deserializer).map(Self)
    }
}

trait Tokenize {
    fn tokenize(&self) -> Token;
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationItem {
    #[serde(deserialize_with = "deserialize_bytes_string")]
    pub destination: Vec<u8>,
    #[serde(deserialize_with = "deserialize_u256_string")]
    pub amount: U256,
}

impl Tokenize for AllocationItem {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![
            Token::FixedBytes(self.destination.clone()),
            Token::Uint(self.amount),
        ])
    }
}

#[repr(u8)]
enum AssetOutcomeType {
    AllocationOutcomeType = 0,
    GuaranteeOutcomeType = 1,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationAssetOutcome {
    pub asset_holder_address: Address,
    pub allocation_items: Vec<AllocationItem>,
}

impl AllocationAssetOutcome {
    fn encode(&self) -> Vec<u8> {
        encode(&[self.tokenize()])
    }

    fn encode_allocation_items(&self) -> Vec<u8> {
        encode(&[Token::Array(
            self.allocation_items
                .iter()
                .map(Tokenize::tokenize)
                .collect::<Vec<_>>(),
        )])
    }
}

impl Tokenize for AllocationAssetOutcome {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![
            Token::Uint(U256::from(AssetOutcomeType::AllocationOutcomeType as u8)),
            Token::Bytes(self.encode_allocation_items()),
        ])
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Guarantee {
    #[serde(deserialize_with = "deserialize_bytes_string")]
    pub target_channel_id: Vec<u8>,
    #[serde(deserialize_with = "deserialize_bytes_strings")]
    pub destinations: Vec<Vec<u8>>,
}

impl Guarantee {
    fn encode(&self) -> Vec<u8> {
        encode(&[self.tokenize()])
    }
}

impl Tokenize for Guarantee {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![
            Token::FixedBytes(self.target_channel_id.clone().into()),
            Token::Array(
                self.destinations
                    .iter()
                    .map(|destination| Token::FixedBytes(destination.clone().into()))
                    .collect(),
            ),
        ])
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteeAssetOutcome {
    pub asset_holder_address: Address,
    pub guarantee: Guarantee,
}

impl GuaranteeAssetOutcome {
    fn encode(&self) -> Vec<u8> {
        encode(&[self.tokenize()])
    }
}

impl Tokenize for GuaranteeAssetOutcome {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![
            Token::Uint(U256::from(AssetOutcomeType::GuaranteeOutcomeType as u8)),
            Token::Bytes(self.guarantee.encode()),
        ])
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum AssetOutcome {
    AllocationAssetOutcome(AllocationAssetOutcome),
    GuaranteeAssetOutcome(GuaranteeAssetOutcome),
}

impl AssetOutcome {
    fn asset_holder_address(&self) -> &Address {
        match self {
            Self::AllocationAssetOutcome(o) => &o.asset_holder_address,
            Self::GuaranteeAssetOutcome(o) => &o.asset_holder_address,
        }
    }
}

impl Tokenize for AssetOutcome {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![
            Token::Address(self.asset_holder_address().clone()),
            Token::Bytes(match self {
                Self::AllocationAssetOutcome(o) => o.encode(),
                Self::GuaranteeAssetOutcome(o) => o.encode(),
            }),
        ])
    }
}

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Outcome(Vec<AssetOutcome>);

impl Outcome {
    pub fn encode(&self) -> Vec<u8> {
        encode(&[self.tokenize()])
    }

    pub fn hash(&self) -> [u8; 32] {
        keccak256(encode(&[Token::Bytes(self.encode())]).as_slice())
    }
}

impl Tokenize for Outcome {
    fn tokenize(&self) -> Token {
        Token::Array(self.0.iter().map(Tokenize::tokenize).collect())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    #[serde(deserialize_with = "deserialize_u256_string")]
    pub chain_id: U256,
    #[serde(deserialize_with = "deserialize_u256_number")]
    pub channel_nonce: U256,
    pub participants: Vec<Address>,
}

impl Channel {
    pub fn id(&self) -> [u8; 32] {
        keccak256(
            encode(&[
                Token::Uint(self.chain_id),
                Token::Array(
                    self.participants
                        .iter()
                        .cloned()
                        .map(Token::Address)
                        .collect(),
                ),
                Token::Uint(self.channel_nonce),
            ])
            .as_slice(),
        )
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    #[serde(deserialize_with = "deserialize_u64")]
    pub turn_num: U64,
    pub is_final: bool,
    pub channel: Channel,
    #[serde(deserialize_with = "deserialize_u64")]
    pub challenge_duration: U64,
    pub outcome: Outcome,
    pub app_definition: Address,
    #[serde(deserialize_with = "deserialize_bytes_string")]
    pub app_data: Vec<u8>,
}

impl State {
    pub fn hash_app_part(&self) -> [u8; 32] {
        keccak256(
            encode(&[
                Token::Uint(self.challenge_duration.as_u64().into()),
                Token::Address(self.app_definition),
                Token::Bytes(self.app_data.clone()),
            ])
            .as_slice(),
        )
    }

    pub fn hash(&self) -> [u8; 32] {
        keccak256(
            encode(&[
                Token::Uint(self.turn_num.as_u64().into()),
                Token::Bool(self.is_final),
                Token::FixedBytes(self.channel.id().into()),
                Token::FixedBytes(self.hash_app_part().into()),
                Token::FixedBytes(self.outcome.hash().into()),
            ])
            .as_slice(),
        )
    }

    pub fn sign(self, private_key: Bytes) -> StateSignature {
        let hash = self.hash().into();
        let hashed_message = hash_message(&hash);
        let message = Message::parse(&hashed_message);
        let secret_key = SecretKey::parse_slice(private_key.deref()).expect("invalid private key");
        let (mut signature, recovery_id) = sign(&message, &secret_key);

        signature.normalize_s();

        StateSignature {
            hash: hash.into(),
            signature: (signature, recovery_id),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateSignature {
    // #[serde(serialize_with = "serialize_bytes_string")]
    hash: Vec<u8>,
    #[serde(serialize_with = "serialize_signature")]
    signature: (Signature, RecoveryId),
}

pub fn hash_message(msg: &Vec<u8>) -> [u8; 32] {
    const PREFIX: &str = "\x19Ethereum Signed Message:\n";
    let mut eth_msg = format!("{}{}", PREFIX, msg.len()).into_bytes();
    eth_msg.extend_from_slice(msg.as_slice());
    keccak256(&eth_msg)
}
