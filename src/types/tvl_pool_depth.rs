use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/*

*** TVL Pool Depth Scheme ***

{
		"pool": "ETH.YFI-0X0BC529C00C6401AEF6D220BE8C6EA1667F6AD93E",
		"totalDepth": "3428747071150"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TVLPoolDepth {
	pool: String,

	#[serde(rename = "totalDepth", deserialize_with = "deserialize_number_from_string")]
	total_depth: u64,
}

impl TVLPoolDepth {
	#[must_use]
	pub fn get_pool(&self) -> &str {
		&self.pool
	}

	#[must_use]
	pub const fn get_total_depth(&self) -> &u64 {
		&self.total_depth
	}
}
