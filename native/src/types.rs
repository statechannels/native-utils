use std::ops::Deref;

use ethabi::{encode, Token};
use ethereum_types::{Address, U256, U64};
use secp256k1::{recover, sign, Message, PublicKey, RecoveryId, SecretKey, Signature};
use serde::de::*;
use serde::ser::*;
use serde_derive::*;

use super::keccak256;
use super::serde::*;

pub struct Bytes(Vec<u8>);

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
        serialize_bytes_string(self.deref(), serializer)
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

fn public_key_to_address(public_key: PublicKey) -> Vec<u8> {
    let uncompressed = public_key.serialize();
    debug_assert_eq!(uncompressed[0], 0x04);
    let hash = keccak256(&uncompressed[1..]);
    hash[12..].into()
}

fn checksum_address(address: Vec<u8>) -> String {
    let s = hex::encode(&address);
    let unchecksummed = s.as_bytes();
    let hash = keccak256(unchecksummed);

    let mut result = String::with_capacity(42);
    result.push('0');
    result.push('x');
    result.extend(unchecksummed.into_iter().enumerate().map(|(i, c)| {
        let mut hash_byte = hash[i / 2];
        if i % 2 == 0 {
            hash_byte = hash_byte >> 4;
        } else {
            hash_byte = hash_byte & 0xf;
        }
        if *c > 57 && hash_byte > 7 {
            char::from(c - 32)
        } else {
            char::from(*c)
        }
    }));

    result
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

    pub fn recover_address(self, signature: Bytes) -> String {
        let hash = self.hash().into();
        let hashed_message = hash_message(&hash);
        let message = Message::parse(&hashed_message);
        let parsed_signature = Signature::parse_slice(&signature[0..signature.len() - 1])
            .expect("invalid signature length");
        let recovery_id =
            RecoveryId::parse_rpc(signature[signature.len() - 1]).expect("invalid recovery ID");
        let public_key = recover(&message, &parsed_signature, &recovery_id)
            .expect("failed to recover signature");

        checksum_address(public_key_to_address(public_key))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateSignature {
    #[serde(serialize_with = "serialize_bytes_string")]
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
