use std::ops::Deref;

use neon::prelude::*;
use neon_serde::*;

use statechannels_native_utils_common::prelude::*;

export! {
  fn getChannelId(channel: Channel) -> String {
    channel.id().to_hex_string()
  }

  fn encodeOutcome(state: State) -> String {
    state.outcome.encode().to_hex_string()
  }

  fn hashAppPart(state: State) -> String {
    state.hash_app_part().to_hex_string()
  }

  fn hashOutcome(state: State) -> String {
    state.outcome.hash().to_hex_string()
  }

  fn hashState(state: State) -> String {
    state.hash().to_hex_string()
  }

  fn hashMessage(msg: Bytes) -> String {
    Bytes32::from(hash_message(&msg.deref())).to_hex_string()
  }

  fn signState(state: State, private_key: Bytes) -> Result<StateSignature, &'static str> {
    state.sign(private_key)
  }

  fn recoverAddress(state: State, signature: Bytes) -> Result<String, &'static str> {
    state.recover_address(signature)
  }

  fn verifySignature(hash: Bytes32, address: String, signature: Bytes) -> Result<bool, &'static str> {
    verify_sig(hash, address, signature)
  }
}
