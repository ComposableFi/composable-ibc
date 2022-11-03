use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, PartialEq)]
pub struct Height {
	/// Previously known as "epoch"
	pub revision_number: u64,
	/// The height of a block
	pub revision_height: u64,
}

impl From<Height> for ibc::Height {
	fn from(value: Height) -> Self {
		Self { revision_number: value.revision_number, revision_height: value.revision_height }
	}
}
