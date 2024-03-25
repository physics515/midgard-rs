use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::AssetAmount;

/*

*** Action Metadata Swap Streaming Swap Meta Scheme ***
{
		"count": "1",
		"depositedCoin": AssetAmount,
		"inCoin": AssetAmount,
		"interval": "1",
		"lastHeight": "15102881",
		"outCoin": AssetAmount,
		"quantity": "1"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionMetadataSwapStreamingSwapMeta {
	#[serde(deserialize_with = "deserialize_option_number_from_string")]
	count: Option<u64>,

	#[serde(rename = "depositedCoin")]
	deposited_coin: AssetAmount,

	#[serde(rename = "inCoin")]
	in_coin: AssetAmount,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	interval: u64,

	#[serde(rename = "lastHeight", deserialize_with = "deserialize_number_from_string")]
	last_height: u64,

	#[serde(rename = "outCoin")]
	out_coin: AssetAmount,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	quantity: u64,
}

impl ActionMetadataSwapStreamingSwapMeta {
	#[must_use]
	pub const fn get_count(&self) -> &Option<u64> {
		&self.count
	}

	#[must_use]
	pub const fn get_deposited_coin(&self) -> &AssetAmount {
		&self.deposited_coin
	}

	#[must_use]
	pub const fn get_in_coin(&self) -> &AssetAmount {
		&self.in_coin
	}

	#[must_use]
	pub const fn get_interval(&self) -> &u64 {
		&self.interval
	}

	#[must_use]
	pub const fn get_last_height(&self) -> &u64 {
		&self.last_height
	}

	#[must_use]
	pub const fn get_out_coin(&self) -> &AssetAmount {
		&self.out_coin
	}

	#[must_use]
	pub const fn get_quantity(&self) -> &u64 {
		&self.quantity
	}
}
