use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/*

*** Action Metadata Swap Scheme ***
{
		"liquidityUnits": "string"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionMetadataAddLiquidity {
	#[serde(rename = "liquidityUnits", deserialize_with = "deserialize_number_from_string")]
	liquidity_units: i64,
}

impl ActionMetadataAddLiquidity {
	#[must_use]
	pub const fn get_liquidity_units(&self) -> &i64 {
		&self.liquidity_units
	}
}
