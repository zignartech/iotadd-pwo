use serde::{Deserialize,Serialize};
use validator::Validate;

pub use iota_streams::ddml::types::typenum;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SubscriptorSchema {
  // pub seed: String,
  pub address: Address,
  pub subscriptor: Subscriber,
}
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Subscriber {
  pub password: String,
  pub state: String,
  // pub seed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Address {
  pub appInst: String,
  pub msgId: String,
}