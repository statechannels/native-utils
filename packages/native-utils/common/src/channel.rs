use ethabi::{encode, Token};
use ethereum_types::{Address};
use serde_derive::*;

use super::tokenize::*;
use super::types::*;
use super::utils::*;
use super::state::*;

//#[derive(Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct SignedStateVarsWithHash {
    pub turn_num: Uint48,
    pub is_final: bool,
    pub outcome: Outcome,
    pub app_data: Bytes,
    pub hash: Bytes32,
    pub signature: RecoverableSignature
}

#[derive(Deserialize, Serialize, Clone)]
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