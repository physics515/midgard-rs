use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::AssetAmounts;

/*

*** Action Out Scheme ***

{
		"address": "thor1dl7un46w7l7f3ewrnrm6nq58nerjtp0dradjtd",
		"coins": AssetAmounts,
		"height": "15102881",
		"txID": ""
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionOut {
	address: String,

	coins: AssetAmounts,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	height: u64,

	#[serde(rename = "txID")]
	tx_id: String,
}

impl ActionOut {
	#[must_use]
	pub const fn get_address(&self) -> &String {
		&self.address
	}

	#[must_use]
	pub const fn get_coins(&self) -> &AssetAmounts {
		&self.coins
	}

	#[must_use]
	pub const fn get_height(&self) -> &u64 {
		&self.height
	}

	#[must_use]
	pub const fn get_tx_id(&self) -> &String {
		&self.tx_id
	}
}
