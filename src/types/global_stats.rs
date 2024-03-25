use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/*

*** Global Stats Scheme ***

{
		"addLiquidityCount": "433683",
		"addLiquidityVolume": "26779919923614488",
		"dailyActiveUsers": "0",
		"monthlyActiveUsers": "0",
		"runeDepth": "2737937863265687",
		"runePriceUSD": "8.395018198401019",
		"swapCount": "26301058",
		"swapCount24h": "103017",
		"swapCount30d": "2988416",
		"swapVolume": "1392828903646971093",
		"switchedRune": "48627225460937419",
		"synthBurnCount": "8978295",
		"synthMintCount": "9962234",
		"toAssetCount": "3256735",
		"toRuneCount": "4086521",
		"uniqueSwapperCount": "0",
		"withdrawCount": "200035",
		"withdrawVolume": "27763612671159391"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GlobalStats {
	#[serde(rename = "addLiquidityCount", deserialize_with = "deserialize_number_from_string")]
	add_liquidity_count: u64,

	#[serde(rename = "addLiquidityVolume", deserialize_with = "deserialize_number_from_string")]
	add_liquidity_volume: u64,

	#[serde(rename = "dailyActiveUsers", deserialize_with = "deserialize_number_from_string")]
	daily_active_users: u64,

	#[serde(rename = "monthlyActiveUsers", deserialize_with = "deserialize_number_from_string")]
	monthly_active_users: u64,

	#[serde(rename = "runeDepth", deserialize_with = "deserialize_number_from_string")]
	rune_depth: u64,

	#[serde(rename = "runePriceUSD", with = "rust_decimal::serde::str")]
	rune_price_usd: Decimal,

	#[serde(rename = "swapCount", deserialize_with = "deserialize_number_from_string")]
	swap_count: u64,

	#[serde(rename = "swapCount24h", deserialize_with = "deserialize_number_from_string")]
	swap_count_24h: u64,

	#[serde(rename = "swapCount30d", deserialize_with = "deserialize_number_from_string")]
	swap_count_30d: u64,

	#[serde(rename = "swapVolume", deserialize_with = "deserialize_number_from_string")]
	swap_volume: u64,

	#[serde(rename = "switchedRune", deserialize_with = "deserialize_number_from_string")]
	switched_rune: u64,

	#[serde(rename = "synthBurnCount", deserialize_with = "deserialize_number_from_string")]
	synth_burn_count: u64,

	#[serde(rename = "synthMintCount", deserialize_with = "deserialize_number_from_string")]
	synth_mint_count: u64,

	#[serde(rename = "toAssetCount", deserialize_with = "deserialize_number_from_string")]
	to_asset_count: u64,

	#[serde(rename = "toRuneCount", deserialize_with = "deserialize_number_from_string")]
	to_rune_count: u64,

	#[serde(rename = "uniqueSwapperCount", deserialize_with = "deserialize_number_from_string")]
	unique_swapper_count: u64,

	#[serde(rename = "withdrawCount", deserialize_with = "deserialize_number_from_string")]
	withdraw_count: u64,

	#[serde(rename = "withdrawVolume", deserialize_with = "deserialize_number_from_string")]
	withdraw_volume: u64,
}

impl GlobalStats {
	#[must_use]
	pub const fn get_add_liquidity_count(&self) -> &u64 {
		&self.add_liquidity_count
	}

	#[must_use]
	pub const fn get_add_liquidity_volume(&self) -> &u64 {
		&self.add_liquidity_volume
	}

	#[must_use]
	pub const fn get_daily_active_users(&self) -> &u64 {
		&self.daily_active_users
	}

	#[must_use]
	pub const fn get_monthly_active_users(&self) -> &u64 {
		&self.monthly_active_users
	}

	#[must_use]
	pub const fn get_rune_depth(&self) -> &u64 {
		&self.rune_depth
	}

	#[must_use]
	pub const fn get_rune_price_usd(&self) -> &Decimal {
		&self.rune_price_usd
	}

	#[must_use]
	pub const fn get_swap_count(&self) -> &u64 {
		&self.swap_count
	}

	#[must_use]
	pub const fn get_swap_count_24h(&self) -> &u64 {
		&self.swap_count_24h
	}

	#[must_use]
	pub const fn get_swap_count_30d(&self) -> &u64 {
		&self.swap_count_30d
	}

	#[must_use]
	pub const fn get_swap_volume(&self) -> &u64 {
		&self.swap_volume
	}

	#[must_use]
	pub const fn get_switched_rune(&self) -> &u64 {
		&self.switched_rune
	}

	#[must_use]
	pub const fn get_synth_burn_count(&self) -> &u64 {
		&self.synth_burn_count
	}

	#[must_use]
	pub const fn get_synth_mint_count(&self) -> &u64 {
		&self.synth_mint_count
	}

	#[must_use]
	pub const fn get_to_asset_count(&self) -> &u64 {
		&self.to_asset_count
	}

	#[must_use]
	pub const fn get_to_rune_count(&self) -> &u64 {
		&self.to_rune_count
	}

	#[must_use]
	pub const fn get_unique_swapper_count(&self) -> &u64 {
		&self.unique_swapper_count
	}

	#[must_use]
	pub const fn get_withdraw_count(&self) -> &u64 {
		&self.withdraw_count
	}

	#[must_use]
	pub const fn get_withdraw_volume(&self) -> &u64 {
		&self.withdraw_volume
	}
}
