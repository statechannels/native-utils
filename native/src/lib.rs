use std::ops::Deref;

use neon::prelude::*;
use neon_serde::*;

mod keccak;
pub use keccak::keccak256;

mod serde;
pub use crate::serde::*;

mod types;
use types::*;

export! {
  fn getChannelId(channel: Channel) -> String {
    format!("0x{}", hex::encode(channel.id()))
  }

  fn encodeOutcome(state: State) -> String {
    format!("0x{}", hex::encode( state.outcome.encode()))
  }

  fn hashAppPart(state: State) -> String {
    format!("0x{}", hex::encode( state.hash_app_part()))
  }

  fn hashOutcome(state: State) -> String {
    format!("0x{}", hex::encode( state.outcome.hash()))
  }

  fn hashState(state: State) -> String {
    format!("0x{}", hex::encode(state.hash()))
  }

  fn hashMessage(msg: Bytes) -> String {
    let bytes = hash_message(&msg.deref());
    format!("0x{}", hex::encode(bytes))
  }

  fn signState(state: State, private_key: Bytes) -> StateSignature {
    state.sign(private_key)
  }

  fn recoverAddress(state: State, signature: Bytes) -> String {
    state.recover_address(signature)
  }
}
