use std::ops::Deref;

use ethabi::{encode, Token};
use ethereum_types::{Address, U256};
use secp256k1::{recover, sign, Message, RecoveryId, SecretKey, Signature};
use serde_derive::*;

use super::encode::*;
use super::serde::*;
use super::tokenize::*;
use super::types::*;
use super::utils::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationItem {
    pub destination: Bytes32,
    pub amount: Uint256,
}

impl Tokenize for AllocationItem {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![self.destination.tokenize(), self.amount.tokenize()])
    }
}

#[repr(u8)]
enum AssetOutcomeType {
    AllocationOutcomeType = 0,
    GuaranteeOutcomeType = 1,
}

impl Tokenize for AssetOutcomeType {
    fn tokenize(&self) -> Token {
        Token::Uint(U256::from(match self {
            Self::AllocationOutcomeType => 0,
            Self::GuaranteeOutcomeType => 1,
        }))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllocationAssetOutcome {
    pub asset_holder_address: Address,
    pub allocation_items: Vec<AllocationItem>,
}

impl Tokenize for AllocationAssetOutcome {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![
            AssetOutcomeType::AllocationOutcomeType.tokenize(),
            Token::Bytes(self.allocation_items.encode()),
        ])
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Guarantee {
    pub target_channel_id: Bytes32,
    pub destinations: Vec<Bytes32>,
}

impl Tokenize for Guarantee {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![
            self.target_channel_id.tokenize(),
            self.destinations.tokenize(),
        ])
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuaranteeAssetOutcome {
    pub asset_holder_address: Address,
    pub guarantee: Guarantee,
}

impl Tokenize for GuaranteeAssetOutcome {
    fn tokenize(&self) -> Token {
        Token::Tuple(vec![
            AssetOutcomeType::GuaranteeOutcomeType.tokenize(),
            Token::Bytes(self.guarantee.encode()),
        ])
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AssetOutcome {
    AllocationAssetOutcome(AllocationAssetOutcome),
    GuaranteeAssetOutcome(GuaranteeAssetOutcome),
}

impl Tokenize for AssetOutcome {
    fn tokenize(&self) -> Token {
        let (asset_holder_address, encoded) = match self {
            Self::AllocationAssetOutcome(o) => (o.asset_holder_address, o.encode()),
            Self::GuaranteeAssetOutcome(o) => (o.asset_holder_address, o.encode()),
        };
        Token::Tuple(vec![
            asset_holder_address.tokenize(),
            Bytes(encoded).tokenize(),
        ])
    }
}

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Outcome(Vec<AssetOutcome>);

impl Outcome {
    pub fn hash(&self) -> Bytes32 {
        keccak256(encode(&[Token::Bytes(self.encode())]).as_slice()).into()
    }
}

impl Tokenize for Outcome {
    fn tokenize(&self) -> Token {
        self.0.tokenize()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub chain_id: Uint256,
    pub channel_nonce: Uint256,
    pub participants: Vec<Address>,
}

impl Channel {
    pub fn id(&self) -> Bytes32 {
        keccak256(
            encode(&[
                self.chain_id.tokenize(),
                Token::Array(
                    self.participants
                        .iter()
                        .cloned()
                        .map(Token::Address)
                        .collect(),
                ),
                self.channel_nonce.tokenize(),
            ])
            .as_slice(),
        )
        .into()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub turn_num: Uint48,
    pub is_final: bool,
    pub channel: Channel,
    pub challenge_duration: Uint48,
    pub outcome: Outcome,
    pub app_definition: Address,
    pub app_data: Bytes,
}

impl State {
    pub fn hash_app_part(&self) -> Bytes32 {
        keccak256(
            encode(&[
                self.challenge_duration.tokenize(),
                self.app_definition.tokenize(),
                self.app_data.tokenize(),
            ])
            .as_slice(),
        )
        .into()
    }

    pub fn hash(&self) -> Bytes32 {
        keccak256(
            encode(&[
                self.turn_num.tokenize(),
                self.is_final.tokenize(),
                self.channel.id().tokenize(),
                self.hash_app_part().tokenize(),
                self.outcome.hash().tokenize(),
            ])
            .as_slice(),
        )
        .into()
    }

    pub fn sign(self, private_key: Bytes) -> StateSignature {
        let hash = self.hash();
        let hashed_message = hash_message(&hash);
        let message = Message::parse(&hashed_message);
        let secret_key = SecretKey::parse_slice(private_key.deref()).expect("invalid private key");
        let (mut signature, recovery_id) = sign(&message, &secret_key);

        signature.normalize_s();

        StateSignature {
            hash,
            signature: (signature, recovery_id),
        }
    }

    pub fn recover_address(self, signature: Bytes) -> String {
        let hash = self.hash();
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
    hash: Bytes32,
    #[serde(serialize_with = "serialize_signature")]
    signature: (Signature, RecoveryId),
}
