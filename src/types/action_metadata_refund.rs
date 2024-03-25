use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::AssetAmounts;

/*

*** Action Metadata Refund Scheme ***
{
		"networkFees": AssetAmounts,
		"reason": "string",
		"memo": "string",
		"affiliateFee": "string",
		"affiliateAddress": "string"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionMetadataRefund {
	#[serde(rename = "networkFees")]
	network_fees: AssetAmounts,

	reason: String,

	memo: String,

	#[serde(rename = "affiliateFee", deserialize_with = "deserialize_number_from_string")]
	affiliate_fee: u64,

	#[serde(rename = "affiliateAddress")]
	affiliate_address: String,
}

impl ActionMetadataRefund {
	#[must_use]
	pub const fn get_network_fees(&self) -> &AssetAmounts {
		&self.network_fees
	}

	#[must_use]
	pub const fn get_reason(&self) -> &String {
		&self.reason
	}

	#[must_use]
	pub const fn get_memo(&self) -> &String {
		&self.memo
	}

	#[must_use]
	pub const fn get_affiliate_fee(&self) -> &u64 {
		&self.affiliate_fee
	}

	#[must_use]
	pub const fn get_affiliate_address(&self) -> &String {
		&self.affiliate_address
	}
}
