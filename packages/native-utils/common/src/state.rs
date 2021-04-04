use std::ops::Deref;

use ethabi::{encode, Token};
use ethereum_types::{Address, U256};
use secp256k1::{recover, sign, verify, Message, RecoveryId, SecretKey, Signature};
use serde_derive::*;

use super::encode::*;
use super::tokenize::*;
use super::types::*;
use super::utils::*;
use super::channel::*;

#[derive(Deserialize,PartialEq)]
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

#[derive(Deserialize,PartialEq)]
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

#[derive(Deserialize, Serialize, PartialEq)]
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

#[derive(Deserialize,PartialEq)]
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

#[derive(Deserialize, PartialEq)]
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

#[derive(Deserialize, PartialEq)]
#[serde(transparent)]
pub struct Outcome(Vec<AssetOutcome>);

impl Outcome {
    pub fn hash(&self) -> Bytes32 {
        keccak256(&self.encode()).into()
    }
}

impl Tokenize for Outcome {
    fn tokenize(&self) -> Token {
        self.0.tokenize()
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

pub enum Status {
    True,
    NeedToCheckApp,
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

    pub fn sign(self, private_key: Bytes) -> Result<StateSignature, &'static str> {
        let hash = self.hash();
        let hashed_message = hash_message(&hash);
        let message = Message::parse(&hashed_message);
        let secret_key =
            SecretKey::parse_slice(private_key.deref()).or_else(|_| Err("invalid private key"))?;
        let (mut signature, recovery_id) = sign(&message, &secret_key);

        signature.normalize_s();

        Ok(StateSignature {
            hash,
            signature: RecoverableSignature(signature, recovery_id),
        })
    }

    pub fn recover_address(self, signature: Bytes) -> Result<String, &'static str> {
        let hash = self.hash();
        let hashed_message = hash_message(&hash);
        let message = Message::parse(&hashed_message);
        let parsed_signature = Signature::parse_slice(&signature[0..signature.len() - 1])
            .or_else(|_| Err("invalid signature length"))?;
        let recovery_id = RecoveryId::parse_rpc(signature[signature.len() - 1])
            .or_else(|_| Err("invalid recovery ID"))?;
        let public_key = recover(&message, &parsed_signature, &recovery_id)
            .or_else(|_| Err("invalid signature"))?;

        Ok(checksum_address(public_key_to_address(public_key)))
    }

    pub fn verify(self, signature: RecoverableSignature) -> Result<bool, &'static str> {
        let hash = self.hash();
        let hashed_message = hash_message(&hash);
        let message = Message::parse(&hashed_message);
        match recover(&message, &signature.0, &signature.1)
        {
            Ok(pubkey) => Ok(verify(&message, &signature.0, &pubkey)),
            Err(_error) => Ok(false)
        }
    }

    fn _require_extra_implicit_checks(&self, to_state: &State) -> Result<(), &'static str> {
        if &self.turn_num.0 + 1 != to_state.turn_num.0 {
            Err("turnNum must increment by one")
        } else if self.channel.chain_id != to_state.channel.chain_id {
            Err("chainId must not change")
        } else if self.channel.channel_nonce != to_state.channel.channel_nonce {
            Err("channelNonce must not change")
        } else if self.app_definition != to_state.app_definition {
            Err("appDefinition must not change")
        } else if self.challenge_duration != to_state.challenge_duration {
            Err("challengeDuration must not change")
        } else {
            Ok(())
        }
    }

    pub fn require_valid_protocol_transition(self, to_state: State) -> Result<Status, &'static str> {
        self._require_extra_implicit_checks(&to_state)?;

        if to_state.is_final {
            if self.outcome != to_state.outcome {
                Err("Outcome change verboten")
            } else {
                Ok(Status::True)
            }
        } else {
            if self.is_final {
                Err("isFinal retrograde")
            } else {
                if to_state.turn_num < Uint48(2 * to_state.channel.participants.len() as u64) {
                    if self.outcome != to_state.outcome {
                        Err("Outcome change verboten")
                    } else if self.app_data != to_state.app_data {
                        Err("appData change forbidden")
                    }
                    else {
                        Ok(Status::True)
                    }
                }
                else {
                    Ok(Status::NeedToCheckApp)
                }
            }
        }
    }
}

pub struct RecoverableSignature(pub Signature, pub RecoveryId);

impl RecoverableSignature {
    pub fn as_bytes(self) -> Bytes {
        let mut v = self.0.serialize().to_vec();
        v.push(self.1.serialize());
        Bytes(v)
    }

    pub fn from_bytes(bytes: Bytes) -> Result<RecoverableSignature,  &'static str> {
        assert_eq!(65,bytes.0.len());
        let mut a: [u8; 64] = [0; 64];
        a.copy_from_slice(&bytes.0[0..64]);
        Ok(RecoverableSignature(
            Signature::parse(&a),
            RecoveryId::parse(bytes.0[64] - 27).map_err(|_| "Invalid recovery ID")?
        ))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateSignature {
    hash: Bytes32,
    //#[serde(serialize_with = "serialize_signature")]
    signature: RecoverableSignature,
}
