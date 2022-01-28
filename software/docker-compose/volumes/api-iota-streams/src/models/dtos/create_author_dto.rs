use serde::{Deserialize,Serialize};
use validator::Validate;
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAuthorQuery {
  pub sendingSeed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateAuthorBody {
  pub seed: String,
}