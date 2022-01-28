use crate::models::schemas::fetch_prevmsg_schema::Subscriber;
use crate::actix_utils::deserialize_json_string::deserializeJsonString;
use crate::models::schemas::fetch_prevmsg_schema::Address;
use serde::{Deserialize,Serialize};
use validator::Validate;
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct FetchPrevMsgQuery {
  #[serde(deserialize_with = "deserializeJsonString")]
  pub address: Address,
  #[serde(deserialize_with = "deserializeJsonString")]
  pub subscriber: Subscriber,
//   pub fetchPrevMsg: bool,
}
