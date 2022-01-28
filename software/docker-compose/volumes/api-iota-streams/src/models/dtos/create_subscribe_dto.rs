use crate::actix_utils::deserialize_json_string::deserializeJsonString;
use crate::models::schemas::subscriptor_schema::Address;
use serde::{Deserialize,Serialize};
use validator::Validate;
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateSubscriberQuery {
  #[serde(deserialize_with = "deserializeJsonString")]
  pub address: Address,
  pub sendingSeed: bool,
//   #[serde(deserialize_with = "deserializeJsonString")]
//   pub author: Author,
//   pub seed: String,
 }
