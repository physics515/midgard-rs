use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/*

*** Network Bond Metrics Scheme ***

{
	"averageActiveBond": "97313077991275",
	"averageStandbyBond": "9627060736882",
	"bondHardCap": "99152180976715",
	"maximumActiveBond": "109057859825816",
	"maximumStandbyBond": "91654298023249",
	"medianActiveBond": "98415406058261",
	"medianStandbyBond": "298000000",
	"minimumActiveBond": "84861177346489",
	"minimumStandbyBond": "53920227428387",
	"totalActiveBond": "9925933955110105",
	"totalStandbyBond": "240676518422057"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkBondMetrics {
	#[serde(rename = "averageActiveBond", deserialize_with = "deserialize_number_from_string")]
	average_active_bond: u64,

	#[serde(rename = "averageStandbyBond", deserialize_with = "deserialize_number_from_string")]
	average_standby_bond: u64,

	#[serde(rename = "bondHardCap", deserialize_with = "deserialize_number_from_string")]
	bond_hard_cap: u64,

	#[serde(rename = "maximumActiveBond", deserialize_with = "deserialize_number_from_string")]
	maximum_active_bond: u64,

	#[serde(rename = "maximumStandbyBond", deserialize_with = "deserialize_number_from_string")]
	maximum_standby_bond: u64,

	#[serde(rename = "medianActiveBond", deserialize_with = "deserialize_number_from_string")]
	median_active_bond: u64,

	#[serde(rename = "medianStandbyBond", deserialize_with = "deserialize_number_from_string")]
	median_standby_bond: u64,

	#[serde(rename = "minimumActiveBond", deserialize_with = "deserialize_number_from_string")]
	minimum_active_bond: u64,

	#[serde(rename = "minimumStandbyBond", deserialize_with = "deserialize_number_from_string")]
	minimum_standby_bond: u64,

	#[serde(rename = "totalActiveBond", deserialize_with = "deserialize_number_from_string")]
	total_active_bond: u64,

	#[serde(rename = "totalStandbyBond", deserialize_with = "deserialize_number_from_string")]
	total_standby_bond: u64,
}

impl NetworkBondMetrics {
	#[must_use]
	pub const fn get_average_active_bond(&self) -> &u64 {
		&self.average_active_bond
	}

	#[must_use]
	pub const fn get_average_standby_bond(&self) -> &u64 {
		&self.average_standby_bond
	}

	#[must_use]
	pub const fn get_bond_hard_cap(&self) -> &u64 {
		&self.bond_hard_cap
	}

	#[must_use]
	pub const fn get_maximum_active_bond(&self) -> &u64 {
		&self.maximum_active_bond
	}

	#[must_use]
	pub const fn get_maximum_standby_bond(&self) -> &u64 {
		&self.maximum_standby_bond
	}

	#[must_use]
	pub const fn get_median_active_bond(&self) -> &u64 {
		&self.median_active_bond
	}

	#[must_use]
	pub const fn get_median_standby_bond(&self) -> &u64 {
		&self.median_standby_bond
	}

	#[must_use]
	pub const fn get_minimum_active_bond(&self) -> &u64 {
		&self.minimum_active_bond
	}

	#[must_use]
	pub const fn get_minimum_standby_bond(&self) -> &u64 {
		&self.minimum_standby_bond
	}

	#[must_use]
	pub const fn get_total_active_bond(&self) -> &u64 {
		&self.total_active_bond
	}

	#[must_use]
	pub const fn get_total_standby_bond(&self) -> &u64 {
		&self.total_standby_bond
	}
}
