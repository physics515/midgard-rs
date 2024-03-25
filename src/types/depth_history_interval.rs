use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

/*

*** Depth History Interval ***

{
		"assetDepth": "131812998956",
		"assetPrice": "7993.962556893128",
		"assetPriceUSD": "72054.12534031394",
		"endTime": "1710172800",
		"liquidityUnits": "384907905455703",
		"luvi": "0.018671610262514394",
		"membersCount": "3640",
		"runeDepth": "1053708178166057",
		"startTime": "1710169200",
		"synthSupply": "102862463587",
		"synthUnits": "246278334048349",
		"units": "631186239504052"
}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepthHistoryInterval {
	#[serde(rename = "assetDepth", deserialize_with = "deserialize_number_from_string")]
	asset_depth: u64,

	#[serde(rename = "assetPrice", with = "rust_decimal::serde::str")]
	asset_price: Decimal,

	#[serde(rename = "assetPriceUSD", with = "rust_decimal::serde::str")]
	asset_price_usd: Decimal,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	#[serde(rename = "liquidityUnits", deserialize_with = "deserialize_number_from_string")]
	liquidity_units: u64,

	#[serde(with = "rust_decimal::serde::str")]
	luvi: Decimal,

	#[serde(rename = "membersCount", deserialize_with = "deserialize_number_from_string")]
	members_count: u64,

	#[serde(rename = "runeDepth", deserialize_with = "deserialize_number_from_string")]
	rune_depth: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,

	#[serde(rename = "synthSupply", deserialize_with = "deserialize_number_from_string")]
	synth_supply: u64,

	#[serde(rename = "synthUnits", deserialize_with = "deserialize_number_from_string")]
	synth_units: u64,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	units: u64,
}

impl DepthHistoryInterval {
	#[must_use]
	pub const fn get_asset_depth(&self) -> &u64 {
		&self.asset_depth
	}

	#[must_use]
	pub const fn get_asset_price(&self) -> &Decimal {
		&self.asset_price
	}

	#[must_use]
	pub const fn get_asset_price_usd(&self) -> &Decimal {
		&self.asset_price_usd
	}

	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_liquidity_units(&self) -> &u64 {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_luvi(&self) -> &Decimal {
		&self.luvi
	}

	#[must_use]
	pub const fn get_members_count(&self) -> &u64 {
		&self.members_count
	}

	#[must_use]
	pub const fn get_rune_depth(&self) -> &u64 {
		&self.rune_depth
	}

	#[must_use]
	pub const fn get_start_time(&self) -> &DateTime<Utc> {
		&self.start_time
	}

	#[must_use]
	pub const fn get_synth_supply(&self) -> &u64 {
		&self.synth_supply
	}

	#[must_use]
	pub const fn get_synth_units(&self) -> &u64 {
		&self.synth_units
	}

	#[must_use]
	pub const fn get_units(&self) -> &u64 {
		&self.units
	}
}
