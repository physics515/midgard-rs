use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/*

*** Earnings Pool Scheme ***

{
		"assetLiquidityFees": "263985540391",
		"earnings": "33867050459",
		"pool": "DOGE.DOGE",
		"rewards": "25227102304",
		"runeLiquidityFees": "4572133578",
		"saverEarning": "237080433101",
		"totalLiquidityFeesRune": "8639948155"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EarningsPool {
	#[serde(rename = "assetLiquidityFees", deserialize_with = "deserialize_number_from_string")]
	asset_liquidity_fees: u64,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	earnings: u64,

	pool: String,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	rewards: i64,

	#[serde(rename = "runeLiquidityFees", deserialize_with = "deserialize_number_from_string")]
	rune_liquidity_fees: u64,

	#[serde(rename = "saverEarning", deserialize_with = "deserialize_number_from_string")]
	saver_earning: u64,

	#[serde(rename = "totalLiquidityFeesRune", deserialize_with = "deserialize_number_from_string")]
	total_liquidity_fees_rune: u64,
}

impl EarningsPool {
	#[must_use]
	pub const fn get_asset_liquidity_fees(&self) -> &u64 {
		&self.asset_liquidity_fees
	}

	#[must_use]
	pub const fn get_earnings(&self) -> &u64 {
		&self.earnings
	}

	#[must_use]
	pub fn get_pool(&self) -> &str {
		&self.pool
	}

	#[must_use]
	pub const fn get_rewards(&self) -> &i64 {
		&self.rewards
	}

	#[must_use]
	pub const fn get_rune_liquidity_fees(&self) -> &u64 {
		&self.rune_liquidity_fees
	}

	#[must_use]
	pub const fn get_saver_earning(&self) -> &u64 {
		&self.saver_earning
	}

	#[must_use]
	pub const fn get_total_liquidity_fees_rune(&self) -> &u64 {
		&self.total_liquidity_fees_rune
	}
}
