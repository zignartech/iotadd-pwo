use crate::models::schemas::author_schema::Author;
use crate::actix_utils::deserialize_json_string::deserializeJsonString;
use crate::models::schemas::author_schema::Address;
use serde::{Deserialize,Serialize};
use validator::Validate;
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SendOneQuery {
  #[serde(deserialize_with = "deserializeJsonString")]
  pub address: Address,
  #[serde(deserialize_with = "deserializeJsonString")]
  pub author: Author,
}
