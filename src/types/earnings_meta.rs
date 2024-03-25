use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

use crate::EarningsPools;

/*

*** Earnings Meta Scheme ***

{
		"avgNodeCount": "102.62",
		"blockRewards": "79432602790702",
		"bondingEarnings": "45803634025196",
		"earnings": "130387195378072",
		"endTime": "1710288000",
		"liquidityEarnings": "84583561352876",
		"liquidityFees": "50954592587370",
		"pools": EarningsPools,
		"runePriceUSD": "9.563065529054409",
		"startTime": "1707696000"
	}

*/

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EarningsMeta {
	#[serde(rename = "avgNodeCount", with = "rust_decimal::serde::str")]
	avg_node_count: Decimal,

	#[serde(rename = "blockRewards", deserialize_with = "deserialize_number_from_string")]
	block_rewards: u64,

	#[serde(rename = "bondingEarnings", deserialize_with = "deserialize_number_from_string")]
	bonding_earnings: u64,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	earnings: u64,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "endTime")]
	end_time: DateTime<Utc>,

	#[serde(rename = "liquidityEarnings", deserialize_with = "deserialize_number_from_string")]
	liquidity_earnings: u64,

	#[serde(rename = "liquidityFees", deserialize_with = "deserialize_number_from_string")]
	liquidity_fees: u64,

	pools: EarningsPools,

	#[serde(rename = "runePriceUSD", with = "rust_decimal::serde::str")]
	rune_price_usd: Decimal,

	#[serde_as(as = "TimestampSeconds<String, Flexible>")]
	#[serde(rename = "startTime")]
	start_time: DateTime<Utc>,
}

impl EarningsMeta {
	#[must_use]
	pub const fn get_avg_node_count(&self) -> &Decimal {
		&self.avg_node_count
	}

	#[must_use]
	pub const fn get_block_rewards(&self) -> &u64 {
		&self.block_rewards
	}

	#[must_use]
	pub const fn get_bonding_earnings(&self) -> &u64 {
		&self.bonding_earnings
	}

	#[must_use]
	pub const fn get_earnings(&self) -> &u64 {
		&self.earnings
	}

	#[must_use]
	pub const fn get_end_time(&self) -> &DateTime<Utc> {
		&self.end_time
	}

	#[must_use]
	pub const fn get_liquidity_earnings(&self) -> &u64 {
		&self.liquidity_earnings
	}

	#[must_use]
	pub const fn get_liquidity_fees(&self) -> &u64 {
		&self.liquidity_fees
	}

	#[must_use]
	pub const fn get_pools(&self) -> &EarningsPools {
		&self.pools
	}

	#[must_use]
	pub const fn get_rune_price_usd(&self) -> &Decimal {
		&self.rune_price_usd
	}

	#[must_use]
	pub const fn get_start_time(&self) -> &DateTime<Utc> {
		&self.start_time
	}
}
