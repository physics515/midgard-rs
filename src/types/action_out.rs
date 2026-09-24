use serde::{Deserialize, Serialize};

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

	/// The THORChain height at which the outbound transaction occurred.
	///
	/// Optional in the Midgard v2 spec (`Transaction` requires only `txID`,
	/// `address` and `coins`) and genuinely absent in practice: 31 of the 40
	/// outbound transactions in a plain `/v2/actions?limit=50` on 2026-09-24
	/// had no height, because the outbound has not landed in a block yet.
	#[serde(default, deserialize_with = "crate::types::optional_number::deserialize")]
	height: Option<u64>,

	/// Whether the transaction is flagged as an affiliate transaction. Declared
	/// in the spec but not currently emitted by the instances sampled.
	#[serde(default)]
	affiliate: Option<bool>,

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
	pub const fn get_height(&self) -> &Option<u64> {
		&self.height
	}

	#[must_use]
	pub const fn get_affiliate(&self) -> &Option<bool> {
		&self.affiliate
	}

	#[must_use]
	pub const fn get_tx_id(&self) -> &String {
		&self.tx_id
	}
}
