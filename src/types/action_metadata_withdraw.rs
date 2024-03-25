use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::AssetAmounts;

/*

*** Action Metadata Withdraw Scheme ***
{
		"liquidityUnits": "string",
		"asymmetry": "string",
		"basisPoints": "string",
		"networkFees": AssetAmounts,
		"impermanentLossProtection": "string",
		"memo": "string"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ActionMetadataWithdraw {
	#[serde(rename = "liquidityUnits", deserialize_with = "deserialize_number_from_string")]
	liquidity_units: i64,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	asymmetry: i64,

	#[serde(rename = "basisPoints", deserialize_with = "deserialize_number_from_string")]
	basis_points: u64,

	#[serde(rename = "networkFees")]
	network_fees: AssetAmounts,

	#[serde(rename = "impermanentLossProtection", deserialize_with = "deserialize_number_from_string")]
	impermanent_loss_protection: i64,

	memo: String,
}

impl ActionMetadataWithdraw {
	#[must_use]
	pub const fn get_liquidity_units(&self) -> &i64 {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_asymmetry(&self) -> &i64 {
		&self.asymmetry
	}

	#[must_use]
	pub const fn get_basis_points(&self) -> &u64 {
		&self.basis_points
	}

	#[must_use]
	pub const fn get_network_fees(&self) -> &AssetAmounts {
		&self.network_fees
	}

	#[must_use]
	pub const fn get_impermanent_loss_protection(&self) -> &i64 {
		&self.impermanent_loss_protection
	}

	#[must_use]
	pub const fn get_memo(&self) -> &String {
		&self.memo
	}
}
