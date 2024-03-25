use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Member Pool Scheme ***

{
		"assetAdded": "360000",
		"assetAddress": "17c57oQDkMkjiHTmDbjH2W1Jj7bRrqtV4X",
		"assetDeposit": "360000",
		"assetPending": "0",
		"assetWithdrawn": "0",
		"dateFirstAdded": "1699829623",
		"dateLastAdded": "1699829623",
		"liquidityUnits": "775667659",
		"pool": "BTC.BTC",
		"runeAdded": "0",
		"runeAddress": "",
		"runeDeposit": "0",
		"runePending": "0",
		"runeWithdrawn": "0"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberPool {
	#[serde(rename = "assetAdded", deserialize_with = "deserialize_number_from_string")]
	asset_added: u64,

	#[serde(rename = "assetAddress")]
	asset_address: String,

	#[serde(rename = "assetDeposit", deserialize_with = "deserialize_number_from_string")]
	asset_deposit: u64,

	#[serde(rename = "assetPending", deserialize_with = "deserialize_number_from_string")]
	asset_pending: u64,

	#[serde(rename = "assetWithdrawn", deserialize_with = "deserialize_number_from_string")]
	asset_withdrawn: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "dateFirstAdded")]
	date_first_added: DateTime<Utc>,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "dateLastAdded")]
	date_last_added: DateTime<Utc>,

	#[serde(rename = "liquidityUnits", deserialize_with = "deserialize_number_from_string")]
	liquidity_units: u64,

	pool: String,

	#[serde(rename = "runeAdded", deserialize_with = "deserialize_number_from_string")]
	rune_added: u64,

	#[serde(rename = "runeAddress")]
	rune_address: String,

	#[serde(rename = "runeDeposit", deserialize_with = "deserialize_number_from_string")]
	rune_deposit: u64,

	#[serde(rename = "runePending", deserialize_with = "deserialize_number_from_string")]
	rune_pending: u64,

	#[serde(rename = "runeWithdrawn", deserialize_with = "deserialize_number_from_string")]
	rune_withdrawn: u64,
}

impl MemberPool {
	#[must_use]
	pub const fn get_asset_added(&self) -> &u64 {
		&self.asset_added
	}

	#[must_use]
	pub const fn get_asset_address(&self) -> &String {
		&self.asset_address
	}

	#[must_use]
	pub const fn get_asset_deposit(&self) -> &u64 {
		&self.asset_deposit
	}

	#[must_use]
	pub const fn get_asset_pending(&self) -> &u64 {
		&self.asset_pending
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
	pub const fn get_liquidity_units(&self) -> &u64 {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_pool(&self) -> &String {
		&self.pool
	}

	#[must_use]
	pub const fn get_rune_added(&self) -> &u64 {
		&self.rune_added
	}

	#[must_use]
	pub const fn get_rune_address(&self) -> &String {
		&self.rune_address
	}

	#[must_use]
	pub const fn get_rune_deposit(&self) -> &u64 {
		&self.rune_deposit
	}

	#[must_use]
	pub const fn get_rune_pending(&self) -> &u64 {
		&self.rune_pending
	}

	#[must_use]
	pub const fn get_rune_withdrawn(&self) -> &u64 {
		&self.rune_withdrawn
	}
}
