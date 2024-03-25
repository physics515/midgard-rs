use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::PoolStatus;

/*

*** Pool Statistics Scheme ***

{
		"addAssetLiquidityVolume": "21559093216525",
		"addLiquidityCount": "204",
		"addLiquidityVolume": "35549551832000",
		"addRuneLiquidityVolume": "13990458615475",
		"annualPercentageRate": "",
		"asset": "BTC.BTC",
		"assetDepth": "130912432302",
		"assetPrice": "8032.090469519714",
		"assetPriceUSD": "68607.11551099402",
		"averageSlip": "0.680060937697664",
		"earnings": "15616236427576",
		"earningsAnnualAsPercentOfDepth": "0.18069182395260117",
		"liquidityUnits": "389213036474226",
		"poolAPY": "",
		"runeDepth": "1051500499834539",
		"saversAPR": "",
		"status": PoolStatus,
		"swapCount": "371527",
		"swapVolume": "33935042500592046",
		"synthSupply": "99800573524",
		"synthUnits": "239739881003189",
		"toAssetAverageSlip": "1.193480577072595",
		"toAssetCount": "34727",
		"toAssetFees": "4281185337731",
		"toAssetVolume": "5129969138619083",
		"toRuneAverageSlip": "0.5035594614898463",
		"toRuneCount": "83861",
		"toRuneFees": "1182791124280",
		"toRuneVolume": "5129599530261148",
		"totalFees": "12165405926607",
		"uniqueMemberCount": "3644",
		"uniqueSwapperCount": "0",
		"units": "628952917477415",
		"withdrawAssetVolume": "55473673757483",
		"withdrawCount": "321",
		"withdrawRuneVolume": "40934592243966",
		"withdrawVolume": "96408266001449"
}

*/

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PoolStatistics {
	#[serde(rename = "addAssetLiquidityVolume", deserialize_with = "deserialize_number_from_string")]
	add_asset_liquidity_volume: u64,

	#[serde(rename = "addLiquidityCount", deserialize_with = "deserialize_number_from_string")]
	add_liquidity_count: u64,

	#[serde(rename = "addLiquidityVolume", deserialize_with = "deserialize_number_from_string")]
	add_liquidity_volume: u64,

	#[serde(rename = "addRuneLiquidityVolume", deserialize_with = "deserialize_number_from_string")]
	add_rune_liquidity_volume: u64,

	#[serde(rename = "annualPercentageRate", with = "rust_decimal::serde::str_option")]
	annual_percentage_rate: Option<Decimal>,

	asset: String,

	#[serde(rename = "assetDepth", deserialize_with = "deserialize_number_from_string")]
	asset_depth: u64,

	#[serde(rename = "assetPrice", with = "rust_decimal::serde::str_option")]
	asset_price: Option<Decimal>,

	#[serde(rename = "assetPriceUSD", with = "rust_decimal::serde::str_option")]
	asset_price_usd: Option<Decimal>,

	#[serde(rename = "averageSlip", with = "rust_decimal::serde::str_option")]
	average_slip: Option<Decimal>,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	earnings: u64,

	#[serde(rename = "earningsAnnualAsPercentOfDepth", with = "rust_decimal::serde::str_option")]
	earnings_annual_as_percent_of_depth: Option<Decimal>,

	#[serde(rename = "liquidityUnits", deserialize_with = "deserialize_number_from_string")]
	liquidity_units: u64,

	#[serde(rename = "poolAPY", with = "rust_decimal::serde::str_option")]
	pool_apy: Option<Decimal>,

	#[serde(rename = "runeDepth", deserialize_with = "deserialize_number_from_string")]
	rune_depth: u64,

	#[serde(rename = "saversAPR", with = "rust_decimal::serde::str_option")]
	savers_apr: Option<Decimal>,

	status: PoolStatus,

	#[serde(rename = "swapCount", deserialize_with = "deserialize_number_from_string")]
	swap_count: u64,

	#[serde(rename = "swapVolume", deserialize_with = "deserialize_number_from_string")]
	swap_volume: u64,

	#[serde(rename = "synthSupply", deserialize_with = "deserialize_number_from_string")]
	synth_supply: u64,

	#[serde(rename = "synthUnits", deserialize_with = "deserialize_number_from_string")]
	synth_units: u64,

	#[serde(rename = "toAssetAverageSlip", with = "rust_decimal::serde::str_option")]
	to_asset_average_slip: Option<Decimal>,

	#[serde(rename = "toAssetCount", deserialize_with = "deserialize_number_from_string")]
	to_asset_count: u64,

	#[serde(rename = "toAssetFees", deserialize_with = "deserialize_number_from_string")]
	to_asset_fees: u64,

	#[serde(rename = "toAssetVolume", deserialize_with = "deserialize_number_from_string")]
	to_asset_volume: u64,

	#[serde(rename = "toRuneAverageSlip", with = "rust_decimal::serde::str_option")]
	to_rune_average_slip: Option<Decimal>,

	#[serde(rename = "toRuneCount", deserialize_with = "deserialize_number_from_string")]
	to_rune_count: u64,

	#[serde(rename = "toRuneFees", deserialize_with = "deserialize_number_from_string")]
	to_rune_fees: u64,

	#[serde(rename = "toRuneVolume", deserialize_with = "deserialize_number_from_string")]
	to_rune_volume: u64,

	#[serde(rename = "totalFees", deserialize_with = "deserialize_number_from_string")]
	total_fees: u64,

	#[serde(rename = "uniqueMemberCount", deserialize_with = "deserialize_number_from_string")]
	unique_member_count: u64,

	#[serde(rename = "uniqueSwapperCount", deserialize_with = "deserialize_number_from_string")]
	unique_swapper_count: u64,

	#[serde(deserialize_with = "deserialize_number_from_string")]
	units: u64,

	#[serde(rename = "withdrawAssetVolume", deserialize_with = "deserialize_number_from_string")]
	withdraw_asset_volume: u64,

	#[serde(rename = "withdrawCount", deserialize_with = "deserialize_number_from_string")]
	withdraw_count: u64,

	#[serde(rename = "withdrawRuneVolume", deserialize_with = "deserialize_number_from_string")]
	withdraw_rune_volume: u64,

	#[serde(rename = "withdrawVolume", deserialize_with = "deserialize_number_from_string")]
	withdraw_volume: u64,
}

impl PoolStatistics {
	#[must_use]
	pub const fn get_add_asset_liquidity_volume(&self) -> &u64 {
		&self.add_asset_liquidity_volume
	}

	#[must_use]
	pub const fn get_add_liquidity_count(&self) -> &u64 {
		&self.add_liquidity_count
	}

	#[must_use]
	pub const fn get_add_liquidity_volume(&self) -> &u64 {
		&self.add_liquidity_volume
	}

	#[must_use]
	pub const fn get_add_rune_liquidity_volume(&self) -> &u64 {
		&self.add_rune_liquidity_volume
	}

	#[must_use]
	pub const fn get_annual_percentage_rate(&self) -> &Option<Decimal> {
		&self.annual_percentage_rate
	}

	#[must_use]
	pub fn get_asset(&self) -> &str {
		&self.asset
	}

	#[must_use]
	pub const fn get_asset_depth(&self) -> &u64 {
		&self.asset_depth
	}

	#[must_use]
	pub const fn get_asset_price(&self) -> &Option<Decimal> {
		&self.asset_price
	}

	#[must_use]
	pub const fn get_asset_price_usd(&self) -> &Option<Decimal> {
		&self.asset_price_usd
	}

	#[must_use]
	pub const fn get_average_slip(&self) -> &Option<Decimal> {
		&self.average_slip
	}

	#[must_use]
	pub const fn get_earnings(&self) -> &u64 {
		&self.earnings
	}

	#[must_use]
	pub const fn get_earnings_annual_as_percent_of_depth(&self) -> &Option<Decimal> {
		&self.earnings_annual_as_percent_of_depth
	}

	#[must_use]
	pub const fn get_liquidity_units(&self) -> &u64 {
		&self.liquidity_units
	}

	#[must_use]
	pub const fn get_pool_apy(&self) -> &Option<Decimal> {
		&self.pool_apy
	}

	#[must_use]
	pub const fn get_rune_depth(&self) -> &u64 {
		&self.rune_depth
	}

	#[must_use]
	pub const fn get_savers_apr(&self) -> &Option<Decimal> {
		&self.savers_apr
	}

	#[must_use]
	pub const fn get_status(&self) -> &PoolStatus {
		&self.status
	}

	#[must_use]
	pub const fn get_swap_count(&self) -> &u64 {
		&self.swap_count
	}

	#[must_use]
	pub const fn get_swap_volume(&self) -> &u64 {
		&self.swap_volume
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
	pub const fn get_to_asset_average_slip(&self) -> &Option<Decimal> {
		&self.to_asset_average_slip
	}

	#[must_use]
	pub const fn get_to_asset_count(&self) -> &u64 {
		&self.to_asset_count
	}

	#[must_use]
	pub const fn get_to_asset_fees(&self) -> &u64 {
		&self.to_asset_fees
	}

	#[must_use]
	pub const fn get_to_asset_volume(&self) -> &u64 {
		&self.to_asset_volume
	}

	#[must_use]
	pub const fn get_to_rune_average_slip(&self) -> &Option<Decimal> {
		&self.to_rune_average_slip
	}

	#[must_use]
	pub const fn get_to_rune_count(&self) -> &u64 {
		&self.to_rune_count
	}

	#[must_use]
	pub const fn get_to_rune_fees(&self) -> &u64 {
		&self.to_rune_fees
	}

	#[must_use]
	pub const fn get_to_rune_volume(&self) -> &u64 {
		&self.to_rune_volume
	}

	#[must_use]
	pub const fn get_total_fees(&self) -> &u64 {
		&self.total_fees
	}

	#[must_use]
	pub const fn get_unique_member_count(&self) -> &u64 {
		&self.unique_member_count
	}

	#[must_use]
	pub const fn get_unique_swapper_count(&self) -> &u64 {
		&self.unique_swapper_count
	}

	#[must_use]
	pub const fn get_units(&self) -> &u64 {
		&self.units
	}

	#[must_use]
	pub const fn get_withdraw_asset_volume(&self) -> &u64 {
		&self.withdraw_asset_volume
	}

	#[must_use]
	pub const fn get_withdraw_count(&self) -> &u64 {
		&self.withdraw_count
	}

	#[must_use]
	pub const fn get_withdraw_rune_volume(&self) -> &u64 {
		&self.withdraw_rune_volume
	}

	#[must_use]
	pub const fn get_withdraw_volume(&self) -> &u64 {
		&self.withdraw_volume
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;

	#[test]
	fn test_pool_statistics() {
		let data = json!({
			"addAssetLiquidityVolume": "24590890355026",
			"addLiquidityCount": "360",
			"addLiquidityVolume": "40221344358897",
			"addRuneLiquidityVolume": "15630454003871",
			"annualPercentageRate": "",
			"asset": "BTC.BTC",
			"assetDepth": "122158911333",
			"assetPrice": "8111.36126510733",
			"assetPriceUSD": "66087.27652984028",
			"averageSlip": "0.7842017800749017",
			"earnings": "13922336835230",
			"earningsAnnualAsPercentOfDepth": "0.1709483244297171",
			"liquidityUnits": "371333607382410",
			"poolAPY": "",
			"runeDepth": "990875061574177",
			"saversAPR": "",
			"status": "available",
			"swapCount": "453127",
			"swapVolume": "42931017080131394",
			"synthSupply": "91549958430",
			"synthUnits": "222530939275319",
			"toAssetAverageSlip": "1.104416718060629",
			"toAssetCount": "42158",
			"toAssetFees": "2675601169560",
			"toAssetVolume": "5359469309862998",
			"toRuneAverageSlip": "0.5682455108466249",
			"toRuneCount": "86709",
			"toRuneFees": "1213533800335",
			"toRuneVolume": "5382511018523059",
			"totalFees": "11903589201397",
			"uniqueMemberCount": "3605",
			"uniqueSwapperCount": "0",
			"units": "593864546657729",
			"withdrawAssetVolume": "96548803913783",
			"withdrawCount": "435",
			"withdrawRuneVolume": "65049683457029",
			"withdrawVolume": "161598487370812"
		})
		.to_string();

		let pool_statistics: PoolStatistics = serde_json::from_str(&data).unwrap();

		println!("pool_statistics: {}", json!(pool_statistics));

		assert_eq!(pool_statistics.get_add_asset_liquidity_volume(), &24590890355026);
	}
}
