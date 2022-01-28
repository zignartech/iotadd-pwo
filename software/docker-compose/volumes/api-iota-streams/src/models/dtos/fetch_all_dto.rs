use crate::models::schemas::subscriptor_schema::Subscriber;
use crate::actix_utils::deserialize_json_string::deserializeJsonString;
use crate::models::schemas::subscriptor_schema::Address;
use serde::{Deserialize,Serialize};
use validator::Validate;
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FetchAllQuery {
  #[serde(deserialize_with = "deserializeJsonString")]
  pub address: Address,
  #[serde(deserialize_with = "deserializeJsonString")]
  pub subscriber: Subscriber,
}
