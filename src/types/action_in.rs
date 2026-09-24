use serde::{Deserialize, Serialize};

use crate::AssetAmounts;

/*

*** Action In Scheme ***
{
		"address": "0x4a23b94bed76c773b32136967661539ae851b6bf",
		"coins": AssetAmounts,
		"txID": "B7125A01993899E2981E3232FE37BAD9214B7570DECC85D6A96C6D992059FA84"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionIn {
	address: String,

	coins: AssetAmounts,

	#[serde(rename = "txID")]
	tx_id: String,

	/// The THORChain height at which the transaction occurred. Optional in the
	/// Midgard v2 spec, and not currently emitted on inbound transactions.
	#[serde(default, deserialize_with = "crate::types::optional_number::deserialize")]
	height: Option<u64>,

	/// Whether the transaction is flagged as an affiliate transaction. Declared
	/// in the spec but not currently emitted by the instances sampled.
	#[serde(default)]
	affiliate: Option<bool>,
}

impl ActionIn {
	#[must_use]
	pub const fn get_address(&self) -> &String {
		&self.address
	}

	#[must_use]
	pub const fn get_coins(&self) -> &AssetAmounts {
		&self.coins
	}

	#[must_use]
	pub const fn get_tx_id(&self) -> &String {
		&self.tx_id
	}

	#[must_use]
	pub const fn get_height(&self) -> &Option<u64> {
		&self.height
	}

	#[must_use]
	pub const fn get_affiliate(&self) -> &Option<bool> {
		&self.affiliate
	}
}
