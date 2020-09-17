use ethabi_next as ethabi;
use neon::register_module;
use neon_serde::export;
use serde_derive::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct AllocationItem {
  destination: String,
  amount: String,
}

type AllocationOutcome = Vec<AllocationItem>;

use ethabi_next::token::Token;

// This should instead be the implementation of `into` on the `From` trait.
fn outcome_as_token(outcome: &AllocationOutcome) -> ethabi::Token {
  Token::Array(
    outcome
      .into_iter()
      .map(|item| {
        Token::Tuple(vec![
          Token::FixedBytes(item.destination.as_str().into()),
          Token::Uint(0.into()), // FIXME: This should be `item.amount.into()`
        ])
      })
      .collect(),
  )
}

#[derive(Debug)]
enum Error {
  Ethabi(ethabi::Error),
}

impl From<ethabi::Error> for Error {
  fn from(err: ethabi::Error) -> Self {
    Error::Ethabi(err)
  }
}

export! {
  fn logOutcome(outcome: AllocationOutcome) -> String {
    format!("The outcome is {:?}", outcome)
  }

  fn logSerializedOutcome(outcome: String) -> String {
    let outcome: AllocationOutcome = serde_json::from_str(&outcome).unwrap();
    format!("The outcome is {:?}", outcome)
  }

  fn encodeOutcome(outcome: String) -> ethabi_next::Bytes {
    let outcome: AllocationOutcome = serde_json::from_str(&outcome).unwrap();
    ethabi_next::encode(&[outcome_as_token(&outcome)])
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use std::str;

//   #[test]
//   fn encode_outcome() {
//     let item = AllocationItem {
//       amount: "0x".to_string(),
//       destination: "0x".to_string(),
//     };

//     let outcome: AllocationOutcome = vec![item];
//     let encoded = encodeOutcome(outcome);
//     let expected = "0000000000000000000000001111111111111111111111111111111111111111";
//     let received = str::from_utf8(&encoded);
//     // assert_eq!(received.unwrap(), expected);
//   }
// }
