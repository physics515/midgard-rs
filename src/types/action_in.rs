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
}
