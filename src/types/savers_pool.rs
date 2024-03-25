use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Savers Pool Scheme ***

{
		"assetAdded": "99999",
		"assetAddress": "bc1qcxssye4j6730h7ehgega3gyykkuwgdgmmpu62n",
		"assetDeposit": "99999",
		"assetRedeem": "103921",
		"assetWithdrawn": "0",
		"dateFirstAdded": "1666948585",
		"dateLastAdded": "1666948585",
		"pool": "BTC.BTC",
		"saverUnits": "99999"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaversPool {
	#[serde(rename = "assetAdded", deserialize_with = "deserialize_number_from_string")]
	asset_added: u64,

	#[serde(rename = "assetAddress")]
	asset_address: String,

	#[serde(rename = "assetDeposit", deserialize_with = "deserialize_number_from_string")]
	asset_deposit: u64,

	#[serde(rename = "assetRedeem", deserialize_with = "deserialize_number_from_string")]
	asset_redeem: u64,

	#[serde(rename = "assetWithdrawn", deserialize_with = "deserialize_number_from_string")]
	asset_withdrawn: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "dateFirstAdded")]
	date_first_added: DateTime<Utc>,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "dateLastAdded")]
	date_last_added: DateTime<Utc>,

	pool: String,

	#[serde(rename = "saverUnits", deserialize_with = "deserialize_number_from_string")]
	saver_units: u64,
}

impl SaversPool {
	#[must_use]
	pub const fn get_asset_added(&self) -> &u64 {
		&self.asset_added
	}

	#[must_use]
	pub fn get_asset_address(&self) -> &str {
		&self.asset_address
	}

	#[must_use]
	pub const fn get_asset_deposit(&self) -> &u64 {
		&self.asset_deposit
	}

	#[must_use]
	pub const fn get_asset_redeem(&self) -> &u64 {
		&self.asset_redeem
	}

	#[must_use]
	pub const fn get_asset_withdrawn(&self) -> &u64 {
		&self.asset_withdrawn
	}

	#[must_use]
	pub const fn get_date_first_added(&self) -> &DateTime<Utc> {
		&self.date_first_added
	}

	#[must_use]
	pub const fn get_date_last_added(&self) -> &DateTime<Utc> {
		&self.date_last_added
	}

	#[must_use]
	pub fn get_pool(&self) -> &str {
		&self.pool
	}

	#[must_use]
	pub const fn get_saver_units(&self) -> &u64 {
		&self.saver_units
	}
}
