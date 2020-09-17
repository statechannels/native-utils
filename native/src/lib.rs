use neon::prelude::*;
use neon_serde::*;

mod keccak;
pub use keccak::keccak256;

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
}
