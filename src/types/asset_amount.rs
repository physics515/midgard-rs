use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/*

*** Asset Amount Scheme ***
{
		"amount": "12000",
		"asset": "BTC.BTC"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssetAmount {
	#[serde(deserialize_with = "deserialize_number_from_string")]
	amount: u64,
	asset: String,
}

impl AssetAmount {
	#[must_use]
	pub const fn get_amount(&self) -> &u64 {
		&self.amount
	}

	#[must_use]
	pub const fn get_asset(&self) -> &String {
		&self.asset
	}
}
