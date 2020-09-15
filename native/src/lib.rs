use neon::register_module;
use neon_serde::export;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct AllocationItem {
  destination: String,
  amount: String,
}

type AllocationOutcome = Vec<AllocationItem>;

export! {
  fn logOutcome(outcome: AllocationOutcome) -> String {
    println!("The first item is {:?}", outcome[0]);
    format!("The outcome is {:?}", outcome)
  }

}
