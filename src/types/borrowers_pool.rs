use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Borrowers Pool Scheme ***

{
		"collateral_asset": "BTC.BTC",
		"collateral_deposited": "46086406",
		"collateral_withdrawn": "28091702",
		"debt_issued_tor": "960997850000",
		"debt_repaid_tor": "316082827470",
		"last_open_loan_timestamp": "1710166153",
		"last_repay_loan_timestamp": "1710086198",
		"target_assets": [
				"BTC.BTC",
				"LTC.LTC",
				"THOR.RUNE"
		]
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BorrowersPool {
	#[serde(rename = "collateral_asset")]
	collateral_asset: String,

	#[serde(rename = "collateral_deposited", deserialize_with = "deserialize_number_from_string")]
	collateral_deposited: u64,

	#[serde(rename = "collateral_withdrawn", deserialize_with = "deserialize_number_from_string")]
	collateral_withdrawn: u64,

	#[serde(rename = "debt_issued_tor", deserialize_with = "deserialize_number_from_string")]
	debt_issued_tor: u64,

	#[serde(rename = "debt_repaid_tor", deserialize_with = "deserialize_number_from_string")]
	debt_repaid_tor: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "last_open_loan_timestamp")]
	last_open_loan_timestamp: DateTime<Utc>,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "last_repay_loan_timestamp")]
	last_repay_loan_timestamp: DateTime<Utc>,

	#[serde(rename = "target_assets")]
	target_assets: Vec<String>,
}

impl BorrowersPool {
	#[must_use]
	pub fn get_collateral_asset(&self) -> &str {
		&self.collateral_asset
	}

	#[must_use]
	pub const fn get_collateral_deposited(&self) -> &u64 {
		&self.collateral_deposited
	}

	#[must_use]
	pub const fn get_collateral_withdrawn(&self) -> &u64 {
		&self.collateral_withdrawn
	}

	#[must_use]
	pub const fn get_debt_issued_tor(&self) -> &u64 {
		&self.debt_issued_tor
	}

	#[must_use]
	pub const fn get_debt_repaid_tor(&self) -> &u64 {
		&self.debt_repaid_tor
	}

	#[must_use]
	pub const fn get_last_open_loan_timestamp(&self) -> &DateTime<Utc> {
		&self.last_open_loan_timestamp
	}

	#[must_use]
	pub const fn get_last_repay_loan_timestamp(&self) -> &DateTime<Utc> {
		&self.last_repay_loan_timestamp
	}

	#[must_use]
	pub const fn get_target_assets(&self) -> &Vec<String> {
		&self.target_assets
	}
}
